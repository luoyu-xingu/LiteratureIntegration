use crate::errors::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CrossrefWork {
    title: Option<Vec<String>>,
    author: Option<Vec<CrossrefAuthor>>,
    abstract_text: Option<String>,
    published_print: Option<CrossrefDate>,
    container_title: Option<Vec<String>>,
    subject: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CrossrefAuthor {
    given: Option<String>,
    family: Option<String>,
    orcid: Option<String>,
    sequence: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrossrefDate {
    date_parts: Vec<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
struct CrossrefResponse {
    message: CrossrefWork,
}

#[derive(Debug, Clone)]
pub struct PaperMeta {
    pub title: String,
    pub authors: Vec<AuthorMeta>,
    pub abstract_text: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub keywords: Vec<String>,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthorMeta {
    pub name: String,
    pub orcid: Option<String>,
    pub is_first: bool,
    pub is_corresponding: bool,
}

pub struct ExternalApiClient {
    client: reqwest::Client,
}

impl ExternalApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .pool_idle_timeout(std::time::Duration::from_secs(30))
                .pool_max_idle_per_host(10)
                .connect_timeout(std::time::Duration::from_secs(5))
                .timeout(std::time::Duration::from_secs(30))
                .tcp_keepalive(std::time::Duration::from_secs(15))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Returns a shared static instance to avoid re-creating the HTTP client on every call.
    /// The reqwest::Client internally maintains a connection pool; reusing it avoids
    /// repeated TLS handshakes and DNS lookups.
    pub fn shared() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<ExternalApiClient> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(ExternalApiClient::new)
    }

    pub async fn fetch_by_identifier(&self, identifier: &str) -> Result<PaperMeta, AppError> {
        let trimmed = identifier.trim();
        let is_doi = trimmed.starts_with("10.")
            || trimmed.as_bytes().get(0..4).map_or(false, |s| s.eq_ignore_ascii_case(b"doi:"));
        if is_doi {
            let doi = if trimmed.as_bytes().get(0..4).map_or(false, |s| s.eq_ignore_ascii_case(b"doi:")) {
                trimmed[4..].trim()
            } else {
                trimmed
            };
            self.fetch_by_doi(doi).await
        } else {
            self.fetch_by_arxiv(trimmed).await
        }
    }

    async fn fetch_by_doi(&self, doi: &str) -> Result<PaperMeta, AppError> {
        let url = format!("https://api.crossref.org/works/{}", doi);
        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "LiteratureIntegration/1.0 (mailto:contact@example.com)",
            )
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalApiError(format!("Crossref request failed: {}", e))
            })?;

        if !resp.status().is_success() {
            return Err(AppError::ImportFailed(format!(
                "Crossref returned status {}",
                resp.status()
            )));
        }

        let body: CrossrefResponse = resp
            .json()
            .await
            .map_err(|e| {
                AppError::ExternalApiError(format!("Failed to parse Crossref response: {}", e))
            })?;

        let work = body.message;
        let title = work
            .title
            .and_then(|t| t.into_iter().next())
            .unwrap_or_default();
        let year = work
            .published_print
            .and_then(|d| d.date_parts.into_iter().next())
            .and_then(|p| p.into_iter().next());

        let mut authors: Vec<AuthorMeta> = Vec::with_capacity(work.author.as_ref().map(|a| a.len()).unwrap_or(0));
        if let Some(crossref_authors) = work.author {
            let total = crossref_authors.len();
            for (i, a) in crossref_authors.into_iter().enumerate() {
                let given = a.given.unwrap_or_default();
                let family = a.family.unwrap_or_default();
                let name = if given.is_empty() {
                    family
                } else {
                    format!("{} {}", given, family)
                };
                authors.push(AuthorMeta {
                    name,
                    orcid: a.orcid,
                    is_first: i == 0,
                    is_corresponding: i == total - 1,
                });
            }
        }

        let keywords = work.subject.unwrap_or_default();
        let journal = work.container_title.and_then(|t| t.into_iter().next());

        Ok(PaperMeta {
            title,
            authors,
            abstract_text: work.abstract_text,
            year,
            journal,
            keywords,
            doi: Some(doi.to_string()),
            arxiv_id: None,
        })
    }

    async fn fetch_by_arxiv(&self, arxiv_id: &str) -> Result<PaperMeta, AppError> {
        let url = format!("http://export.arxiv.org/api/query?id_list={}", arxiv_id);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("arXiv request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(AppError::ImportFailed(format!(
                "arXiv returned status {}",
                resp.status()
            )));
        }

        let body = resp.text().await.map_err(|e| {
            AppError::ExternalApiError(format!("Failed to read arXiv response: {}", e))
        })?;

        let title = extract_xml_tag(&body, "title").unwrap_or_default();
        let summary = extract_xml_tag(&body, "summary");
        let published = extract_xml_tag(&body, "published");
        let year = published.and_then(|p| p.get(..4).and_then(|y| y.parse::<i32>().ok()));

        let author_names = extract_xml_tags(&body, "name");
        let total = author_names.len();
        let authors: Vec<AuthorMeta> = author_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| AuthorMeta {
                name,
                orcid: None,
                is_first: i == 0,
                is_corresponding: i == total - 1,
            })
            .collect();

        Ok(PaperMeta {
            title,
            authors,
            abstract_text: summary,
            year,
            journal: None,
            keywords: Vec::new(),
            doi: None,
            arxiv_id: Some(arxiv_id.to_string()),
        })
    }
}

pub fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let xml_bytes = xml.as_bytes();
    let tag_bytes = tag.as_bytes();
    let tag_len = tag_bytes.len();
    
    // Pre-compute tag patterns to avoid repeated format! allocations
    // Pattern: <tag>
    let mut search_pos = 0usize;
    let xml_len = xml_bytes.len();
    
    while search_pos < xml_len {
        // Find opening tag: <tag>
        let remaining = &xml_bytes[search_pos..];
        let rem_len = remaining.len();
        
        if rem_len < tag_len + 2 {
            break;
        }
        
        // Manual search for "<tag>"
        let mut found_open = None;
        let max_scan = rem_len - tag_len - 1;
        let mut i = 0;
        while i < max_scan {
            if remaining[i] == b'<' {
                // Check if this is our opening tag
                let mut matches = true;
                for j in 0..tag_len {
                    if remaining[i + 1 + j] != tag_bytes[j] {
                        matches = false;
                        break;
                    }
                }
                if matches && remaining[i + 1 + tag_len] == b'>' {
                    found_open = Some(i);
                    break;
                }
            }
            i += 1;
        }
        
        let open_start = match found_open {
            Some(s) => s,
            None => break,
        };
        
        let content_start = search_pos + open_start + tag_len + 2;
        if content_start >= xml_len {
            break;
        }
        
        // Find closing tag: </tag>
        let after_content = &xml_bytes[content_start..];
        let after_len = after_content.len();
        let mut close_found = None;
        
        if after_len >= tag_len + 3 {
            let max_j = after_len - tag_len - 2;
            let mut j = 0;
            while j < max_j {
                if after_content[j] == b'<' && after_content[j + 1] == b'/' {
                    let mut matches = true;
                    for k in 0..tag_len {
                        if after_content[j + 2 + k] != tag_bytes[k] {
                            matches = false;
                            break;
                        }
                    }
                    if matches && after_content[j + 2 + tag_len] == b'>' {
                        close_found = Some(j);
                        break;
                    }
                }
                j += 1;
            }
        }
        
        if let Some(close_offset) = close_found {
            let content_end = content_start + close_offset;
            let content = &xml[content_start..content_end];
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
            search_pos = content_end + tag_len + 3;
        } else {
            search_pos = content_start;
        }
    }
    None
}

pub fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let xml_bytes = xml.as_bytes();
    let tag_bytes = tag.as_bytes();
    let tag_len = tag_bytes.len();
    let xml_len = xml_bytes.len();
    
    let mut results = Vec::with_capacity(32);
    let mut search_pos = 0usize;
    
    while search_pos < xml_len {
        let remaining = &xml_bytes[search_pos..];
        let rem_len = remaining.len();
        
        if rem_len < tag_len + 2 {
            break;
        }
        
        // Find opening tag
        let mut found_open = None;
        let max_scan = rem_len - tag_len - 1;
        let mut i = 0;
        while i < max_scan {
            if remaining[i] == b'<' {
                let mut matches = true;
                for j in 0..tag_len {
                    if remaining[i + 1 + j] != tag_bytes[j] {
                        matches = false;
                        break;
                    }
                }
                if matches && remaining[i + 1 + tag_len] == b'>' {
                    found_open = Some(i);
                    break;
                }
            }
            i += 1;
        }
        
        let open_start = match found_open {
            Some(s) => s,
            None => break,
        };
        
        let content_start = search_pos + open_start + tag_len + 2;
        if content_start >= xml_len {
            break;
        }
        
        // Find closing tag
        let after_content = &xml_bytes[content_start..];
        let after_len = after_content.len();
        let mut close_found = None;
        
        if after_len >= tag_len + 3 {
            let max_j = after_len - tag_len - 2;
            let mut j = 0;
            while j < max_j {
                if after_content[j] == b'<' && after_content[j + 1] == b'/' {
                    let mut matches = true;
                    for k in 0..tag_len {
                        if after_content[j + 2 + k] != tag_bytes[k] {
                            matches = false;
                            break;
                        }
                    }
                    if matches && after_content[j + 2 + tag_len] == b'>' {
                        close_found = Some(j);
                        break;
                    }
                }
                j += 1;
            }
        }
        
        if let Some(close_offset) = close_found {
            let content_end = content_start + close_offset;
            let content = &xml[content_start..content_end];
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                results.push(trimmed.to_string());
            }
            search_pos = content_end + tag_len + 3;
        } else {
            break;
        }
    }
    results.shrink_to_fit();
    results
}
