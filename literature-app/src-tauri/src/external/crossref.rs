use crate::models::paper::Paper;
use once_cell::sync::Lazy;
use super::ImportResult;

static SHARED_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("LiteratureApp/1.0 (mailto:test@example.com)")
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_default()
});

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

fn format_author_name(a: &CrossrefAuthor) -> Option<String> {
    match (&a.given, &a.family) {
        (Some(g), Some(f)) => {
            let mut s = String::with_capacity(g.len() + f.len() + 1);
            s.push_str(g);
            s.push(' ');
            s.push_str(f);
            Some(s)
        }
        (None, Some(f)) => Some(f.clone()),
        (Some(g), None) => Some(g.clone()),
        _ => None,
    }
}

pub async fn fetch_by_doi(doi: &str) -> anyhow::Result<ImportResult> {
    let url = format!("https://api.crossref.org/works/{}", doi);
    let resp = SHARED_CLIENT.get(&url).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("Crossref API error: {}", resp.status());
    }

    let data: CrossrefResponse = resp.json().await?;
    let msg = data.message;

    let title = msg.title.into_iter().next().unwrap_or_default();
    let year = msg
        .published_print
        .as_ref()
        .or(msg.published_online.as_ref())
        .and_then(|d| d.date_parts.first())
        .and_then(|p| p.first().copied());

    let journal = msg.container_title.and_then(|mut v| v.pop());

    let authors = msg.author.unwrap_or_default();
    let first_author = authors.first().and_then(format_author_name);
    let corresponding_author = authors
        .iter()
        .find(|a| a.sequence.as_deref() == Some("corresponding"))
        .or_else(|| authors.first())
        .and_then(format_author_name);

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

    Ok(ImportResult {
        paper,
        abstract_text: msg.abstract_text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_author_both() {
        let a = CrossrefAuthor {
            given: Some("John".into()),
            family: Some("Doe".into()),
            sequence: None,
        };
        assert_eq!(format_author_name(&a), Some("John Doe".into()));
    }

    #[test]
    fn test_format_author_only_family() {
        let a = CrossrefAuthor {
            given: None,
            family: Some("Doe".into()),
            sequence: None,
        };
        assert_eq!(format_author_name(&a), Some("Doe".into()));
    }

    #[test]
    fn test_format_author_none() {
        let a = CrossrefAuthor {
            given: None,
            family: None,
            sequence: None,
        };
        assert_eq!(format_author_name(&a), None);
    }
}
