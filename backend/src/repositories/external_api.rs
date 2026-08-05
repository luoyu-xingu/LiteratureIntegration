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
        // Pre-calculate URL capacity: base URL length + doi length
        let base_len = 28; // "https://api.crossref.org/works/"
        let mut url = String::with_capacity(base_len + doi.len());
        url.push_str("https://api.crossref.org/works/");
        url.push_str(doi);

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
                // Build the "given family" name with a single allocation instead
                // of `format!`, which would allocate a formatting buffer plus the
                // final String.
                let name = if given.is_empty() {
                    family
                } else {
                    let mut n = String::with_capacity(given.len() + 1 + family.len());
                    n.push_str(&given);
                    n.push(' ');
                    n.push_str(&family);
                    n
                };
                authors.push(AuthorMeta {
                    name,
                    orcid: a.orcid,
                    is_first: i == 0,
                    is_corresponding: i == total - 1,
                });
            }
        }

        let mut keywords = work.subject.unwrap_or_default();
        keywords.shrink_to_fit();
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
        // Pre-calculate URL capacity: base URL length + arxiv_id length
        let base_len = 38; // "http://export.arxiv.org/api/query?id_list="
        let mut url = String::with_capacity(base_len + arxiv_id.len());
        url.push_str("http://export.arxiv.org/api/query?id_list=");
        url.push_str(arxiv_id);

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
        let mut authors: Vec<AuthorMeta> = Vec::with_capacity(total);
        for (i, name) in author_names.into_iter().enumerate() {
            authors.push(AuthorMeta {
                name,
                orcid: None,
                is_first: i == 0,
                is_corresponding: i == total - 1,
            });
        }

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

#[inline]
fn find_tag_open(xml: &str, tag: &str, from: usize) -> Option<usize> {
    let haystack = &xml[from..];
    // Search for "<tag>" manually to avoid format! allocation
    let tag_bytes = tag.as_bytes();
    let hay_bytes = haystack.as_bytes();
    let tag_len = tag_bytes.len();
    let hay_len = hay_bytes.len();
    
    if hay_len < tag_len + 2 {
        return None;
    }
    
    let mut i = 0;
    while i + tag_len + 2 <= hay_len {
        if hay_bytes[i] == b'<' {
            let mut matches = true;
            for j in 0..tag_len {
                if hay_bytes[i + 1 + j] != tag_bytes[j] {
                    matches = false;
                    break;
                }
            }
            if matches && hay_bytes[i + 1 + tag_len] == b'>' {
                return Some(from + i);
            }
        }
        i += 1;
    }
    None
}

#[inline]
fn find_tag_close(xml: &str, tag: &str, from: usize) -> Option<usize> {
    let haystack = &xml[from..];
    // Search for "</tag>" manually to avoid format! allocation
    let tag_bytes = tag.as_bytes();
    let hay_bytes = haystack.as_bytes();
    let tag_len = tag_bytes.len();
    let hay_len = hay_bytes.len();
    
    if hay_len < tag_len + 3 {
        return None;
    }
    
    let mut i = 0;
    while i + tag_len + 3 <= hay_len {
        if hay_bytes[i] == b'<' && hay_bytes[i + 1] == b'/' {
            let mut matches = true;
            for j in 0..tag_len {
                if hay_bytes[i + 2 + j] != tag_bytes[j] {
                    matches = false;
                    break;
                }
            }
            if matches && hay_bytes[i + 2 + tag_len] == b'>' {
                return Some(from + i);
            }
        }
        i += 1;
    }
    None
}

#[inline]
pub fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let mut search_pos = 0usize;
    let xml_len = xml.len();
    let tag_end_offset = tag.len() + 2; // len("<tag>")

    while search_pos < xml_len {
        let open_idx = find_tag_open(xml, tag, search_pos)?;

        let content_start = open_idx + tag_end_offset;
        if content_start >= xml_len {
            break;
        }

        let close_idx = find_tag_close(xml, tag, content_start)?;

        let content = &xml[content_start..close_idx];
        let trimmed = content.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
        search_pos = close_idx + tag.len() + 3; // len("</tag>")
    }
    None
}

#[inline]
pub fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let mut results = Vec::with_capacity(32);
    let mut search_pos = 0usize;
    let xml_len = xml.len();
    let tag_end_offset = tag.len() + 2;

    while search_pos < xml_len {
        let open_idx = match find_tag_open(xml, tag, search_pos) {
            Some(i) => i,
            None => break,
        };

        let content_start = open_idx + tag_end_offset;
        if content_start >= xml_len {
            break;
        }

        let close_idx = match find_tag_close(xml, tag, content_start) {
            Some(i) => i,
            None => break,
        };

        let content = &xml[content_start..close_idx];
        let trimmed = content.trim();
        if !trimmed.is_empty() {
            results.push(trimmed.to_string());
        }
        search_pos = close_idx + tag.len() + 3;
    }
    results.shrink_to_fit();
    results
}
