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
    fn create_client() -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client")
    }

    pub fn new() -> Self {
        Self {
            client: Self::create_client(),
        }
    }

    pub fn shared() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<ExternalApiClient> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            client: Self::create_client(),
        })
    }

    pub async fn fetch_by_identifier(&self, identifier: &str) -> Result<PaperMeta, AppError> {
        let trimmed = identifier.trim();
        if trimmed.starts_with("10.") || trimmed.to_lowercase().starts_with("doi:") {
            let doi = trimmed
                .trim_start_matches("doi:")
                .trim_start_matches("DOI:")
                .trim();
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

        let mut authors: Vec<AuthorMeta> = Vec::new();
        if let Some(crossref_authors) = work.author {
            let total = crossref_authors.len();
            for (i, a) in crossref_authors.iter().enumerate() {
                let given = a.given.as_deref().unwrap_or("");
                let family = a.family.as_deref().unwrap_or("");
                let name = if given.is_empty() {
                    family.to_string()
                } else {
                    format!("{} {}", given, family)
                };
                authors.push(AuthorMeta {
                    name,
                    orcid: a.orcid.clone(),
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

        let body = resp.bytes().await.map_err(|e| {
            AppError::ExternalApiError(format!("Failed to read arXiv response: {}", e))
        })?;

        let mut reader = quick_xml::Reader::from_reader(body.as_ref());
        reader.trim_text(true);

        let mut title = String::new();
        let mut summary = None;
        let mut published = String::new();
        let mut author_names = Vec::new();
        let mut in_author = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"title" => {
                            if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                                title = t.unescape().unwrap_or_default().to_string();
                            }
                        }
                        b"summary" => {
                            if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                                summary = Some(t.unescape().unwrap_or_default().to_string());
                            }
                        }
                        b"published" => {
                            if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                                published = t.unescape().unwrap_or_default().to_string();
                            }
                        }
                        b"author" => {
                            in_author = true;
                        }
                        b"name" if in_author => {
                            if let Ok(quick_xml::events::Event::Text(t)) = reader.read_event_into(&mut buf) {
                                author_names.push(t.unescape().unwrap_or_default().to_string());
                            }
                        }
                        _ => {}
                    }
                }
                Ok(quick_xml::events::Event::End(e)) => {
                    if e.name().as_ref() == b"author" {
                        in_author = false;
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }

        let year = published.get(..4).and_then(|y| y.parse::<i32>().ok());
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
