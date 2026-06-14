use crate::errors::AppError;
use once_cell::sync::Lazy;
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

static SHARED_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("LiteratureIntegration/1.0 (mailto:contact@example.com)")
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_default()
});

pub struct ExternalApiClient {
    client: &'static reqwest::Client,
}

impl Default for ExternalApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalApiClient {
    pub fn new() -> Self {
        Self {
            client: &SHARED_CLIENT,
        }
    }

    pub async fn fetch_by_identifier(&self, identifier: &str) -> Result<PaperMeta, AppError> {
        let trimmed = identifier.trim();
        let lower_head = trimmed
            .get(..5.min(trimmed.len()))
            .unwrap_or("")
            .to_ascii_lowercase();
        if trimmed.starts_with("10.") || lower_head == "doi:/" || lower_head.starts_with("doi:") {
            let mut idx = 0usize;
            let bytes = trimmed.as_bytes();
            let prefix = b"doi:";
            while idx + prefix.len() <= bytes.len()
                && bytes[idx..idx + prefix.len()].eq_ignore_ascii_case(prefix)
            {
                idx += prefix.len();
                while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                    idx += 1;
                }
            }
            let doi = trimmed[idx..].trim();
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
            .and_then(|mut t| if t.is_empty() { None } else { Some(t.swap_remove(0)) })
            .unwrap_or_default();
        let year = work
            .published_print
            .and_then(|d| d.date_parts.into_iter().next())
            .and_then(|p| p.into_iter().next());

        let authors: Vec<AuthorMeta> = match work.author {
            Some(crossref_authors) => {
                let total = crossref_authors.len();
                let mut result = Vec::with_capacity(total);
                for (i, a) in crossref_authors.iter().enumerate() {
                    let name = match (&a.given, &a.family) {
                        (Some(g), Some(f)) => {
                            let mut s = String::with_capacity(g.len() + f.len() + 1);
                            s.push_str(g);
                            s.push(' ');
                            s.push_str(f);
                            s
                        }
                        (None, Some(f)) => f.clone(),
                        (Some(g), None) => g.clone(),
                        _ => String::new(),
                    };
                    result.push(AuthorMeta {
                        name,
                        orcid: a.orcid.clone(),
                        is_first: i == 0,
                        is_corresponding: i == total - 1,
                    });
                }
                result
            }
            None => Vec::new(),
        };

        let keywords = work.subject.unwrap_or_default();
        let journal = work
            .container_title
            .and_then(|mut t| if t.is_empty() { None } else { Some(t.swap_remove(0)) });

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

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let mut open_buf = [0u8; 64];
    let open = format_tag_into(&mut open_buf, tag, false);
    let mut close_buf = [0u8; 64];
    let close = format_tag_into(&mut close_buf, tag, true);

    let start = xml.find(open)?;
    let content_start = start + open.len();
    let content_end = xml[content_start..].find(close)?;
    let raw = &xml[content_start..content_start + content_end];
    let trimmed = raw.trim();
    if trimmed.len() == raw.len() {
        Some(trimmed.to_string())
    } else {
        let mut s = String::with_capacity(trimmed.len());
        s.push_str(trimmed);
        Some(s)
    }
}

fn extract_xml_tags(xml: &str, tag: &str) -> Vec<String> {
    let mut open_buf = [0u8; 64];
    let open = format_tag_into(&mut open_buf, tag, false);
    let mut close_buf = [0u8; 64];
    let close = format_tag_into(&mut close_buf, tag, true);

    let mut results: Vec<String> = Vec::with_capacity(8);
    let mut search_from = 0usize;
    while let Some(rel_start) = xml[search_from..].find(open) {
        let content_start = search_from + rel_start + open.len();
        if let Some(rel_end) = xml[content_start..].find(close) {
            let content_end = content_start + rel_end;
            let raw = &xml[content_start..content_end];
            results.push(raw.trim().to_string());
            search_from = content_end + close.len();
        } else {
            break;
        }
    }
    results.shrink_to_fit();
    results
}

fn format_tag_into<'a>(buf: &'a mut [u8; 64], tag: &str, closing: bool) -> &'a str {
    let bytes = buf.as_mut_slice();
    let mut idx = 0;
    bytes[idx] = b'<';
    idx += 1;
    if closing {
        bytes[idx] = b'/';
        idx += 1;
    }
    let tag_bytes = tag.as_bytes();
    if idx + tag_bytes.len() + 1 < bytes.len() {
        bytes[idx..idx + tag_bytes.len()].copy_from_slice(tag_bytes);
        idx += tag_bytes.len();
    }
    bytes[idx] = b'>';
    idx += 1;
    std::str::from_utf8(&bytes[..idx]).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_xml_tag_basic() {
        let xml = "<title>Hello World</title>";
        assert_eq!(extract_xml_tag(xml, "title"), Some("Hello World".to_string()));
    }

    #[test]
    fn test_extract_xml_tag_trimmed() {
        let xml = "<title>\n  Hello\n  World\n</title>";
        assert_eq!(extract_xml_tag(xml, "title"), Some("Hello\n  World".to_string()));
    }

    #[test]
    fn test_extract_xml_tag_missing() {
        let xml = "<foo>bar</foo>";
        assert_eq!(extract_xml_tag(xml, "title"), None);
    }

    #[test]
    fn test_extract_xml_tags_multiple() {
        let xml = "<author><name>A</name></author><author><name>B</name></author><author><name>C</name></author>";
        let names = extract_xml_tags(xml, "name");
        assert_eq!(names, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_extract_xml_tags_empty() {
        let xml = "<nothing>here</nothing>";
        let names = extract_xml_tags(xml, "name");
        assert!(names.is_empty());
    }
}
