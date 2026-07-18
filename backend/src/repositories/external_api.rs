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

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)?;
    let content_start = start + open.len();
    let content_end = xml[content_start..].find(&close)?;
    Some(xml[content_start..content_start + content_end].trim().to_string())
}

fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    // Estimate capacity by counting occurrences of the tag name
    let tag_marker = tag.as_bytes();
    let mut estimated = 0;
    let mut pos = 0;
    while pos + tag_marker.len() <= xml.len() {
        if xml.as_bytes()[pos..].starts_with(tag_marker) {
            estimated += 1;
            pos += tag_marker.len();
        } else {
            pos += 1;
        }
    }
    let mut results = Vec::with_capacity(estimated.min(64));
    let mut search_from = 0;
    let open_len = open_tag.len();
    let close_len = close_tag.len();
    while let Some(start) = xml[search_from..].find(&open_tag) {
        let content_start = search_from + start + open_len;
        if let Some(content_end) = xml[content_start..].find(&close_tag) {
            results.push(
                xml[content_start..content_start + content_end]
                    .trim()
                    .to_string(),
            );
            search_from = content_start + content_end + close_len;
        } else {
            break;
        }
    }
    results
}
