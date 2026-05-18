use crate::models::paper::Paper;
use super::ImportResult;

#[derive(serde::Deserialize)]
struct CrossrefResponse {
    message: CrossrefMessage,
}

#[derive(serde::Deserialize)]
struct CrossrefMessage {
    title: Vec<String>,
    #[serde(rename = "published-print")]
    published_print: Option<CrossrefDate>,
    #[serde(rename = "published-online")]
    published_online: Option<CrossrefDate>,
    #[serde(rename = "container-title")]
    container_title: Option<Vec<String>>,
    author: Option<Vec<CrossrefAuthor>>,
    #[serde(rename = "abstract")]
    abstract_text: Option<String>,
}

#[derive(serde::Deserialize)]
struct CrossrefDate {
    #[serde(rename = "date-parts")]
    date_parts: Vec<Vec<i32>>,
}

#[derive(serde::Deserialize)]
struct CrossrefAuthor {
    given: Option<String>,
    family: Option<String>,
    sequence: Option<String>,
}

pub async fn fetch_by_doi(doi: &str) -> anyhow::Result<ImportResult> {
    let url = format!("https://api.crossref.org/works/{}", doi);
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "LiteratureIntegration/1.0 (mailto:test@example.com)")
        .send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("Crossref API error: {}", resp.status());
    }

    let data: CrossrefResponse = resp.json().await?;
    let msg = data.message;

    let title = msg.title.first().cloned().unwrap_or_default();
    let year = msg.published_print
        .as_ref()
        .or(msg.published_online.as_ref())
        .and_then(|d| d.date_parts.first())
        .and_then(|p| p.first().copied());

    let journal = msg.container_title
        .and_then(|v| v.first().cloned());

    let authors = msg.author.unwrap_or_default();
    let first_author = authors.first().and_then(|a| {
        match (&a.given, &a.family) {
            (Some(g), Some(f)) => Some(format!("{} {}", g, f)),
            (None, Some(f)) => Some(f.clone()),
            (Some(g), None) => Some(g.clone()),
            _ => None,
        }
    });

    let corresponding_author = authors.iter()
        .find(|a| a.sequence.as_deref() == Some("corresponding"))
        .or_else(|| authors.first())
        .and_then(|a| {
            match (&a.given, &a.family) {
                (Some(g), Some(f)) => Some(format!("{} {}", g, f)),
                (None, Some(f)) => Some(f.clone()),
                (Some(g), None) => Some(g.clone()),
                _ => None,
            }
        });

    let abstract_text = msg.abstract_text;

    let paper = Paper {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        doi: Some(doi.to_string()),
        arxiv_id: None,
        year,
        journal,
        first_author,
        corresponding_author,
        keywords: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(ImportResult { paper, abstract_text })
}
