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

pub async fn fetch_by_arxiv_id(arxiv_id: &str) -> anyhow::Result<ImportResult> {
    let url = format!("https://export.arxiv.org/api/query?id_list={}", arxiv_id);
    let resp = SHARED_CLIENT.get(&url).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("arXiv API error: {}", resp.status());
    }

    let body = resp.text().await?;

    let entry = extract_between(&body, "<entry>", "</entry>")
        .ok_or_else(|| anyhow::anyhow!("Paper not found on arXiv: {}", arxiv_id))?;

    let title = extract_between(&entry, "<title>", "</title>")
        .map(collapse_whitespace)
        .unwrap_or_default();

    let abstract_text = extract_between(&entry, "<summary>", "</summary>")
        .map(collapse_whitespace);

    let first_author = extract_between(&entry, "<name>", "</name>").map(|s| s.to_string());

    let year = extract_between(&entry, "<published>", "</published>")
        .and_then(|d| d.get(..4).and_then(|y| y.parse::<i32>().ok()));

    let paper = Paper {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        doi: None,
        arxiv_id: Some(arxiv_id.to_string()),
        year,
        journal: None,
        first_author,
        corresponding_author: None,
        keywords: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(ImportResult { paper, abstract_text })
}

fn collapse_whitespace(s: &str) -> String {
    let trimmed = s.trim();
    let mut result = String::with_capacity(trimmed.len());
    let mut in_ws = false;
    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !in_ws {
                result.push(' ');
                in_ws = true;
            }
        } else {
            result.push(ch);
            in_ws = false;
        }
    }
    result.shrink_to_fit();
    result
}

fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = text.find(start)? + start.len();
    let e = text[s..].find(end)? + s;
    Some(&text[s..e])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_whitespace_basic() {
        assert_eq!(collapse_whitespace("Hello   World"), "Hello World");
    }

    #[test]
    fn test_collapse_whitespace_newlines() {
        assert_eq!(collapse_whitespace("\n  Hello\n  World\n"), "Hello World");
    }

    #[test]
    fn test_collapse_whitespace_empty() {
        assert_eq!(collapse_whitespace("   "), "");
    }

    #[test]
    fn test_extract_between() {
        let s = "<title>Hello</title>";
        assert_eq!(extract_between(s, "<title>", "</title>"), Some("Hello"));
    }
}
