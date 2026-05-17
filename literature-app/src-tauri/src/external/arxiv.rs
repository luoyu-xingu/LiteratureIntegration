use crate::models::paper::Paper;
use super::ImportResult;

pub async fn fetch_by_arxiv_id(arxiv_id: &str) -> anyhow::Result<ImportResult> {
    let url = format!("https://export.arxiv.org/api/query?id_list={}", arxiv_id);
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("arXiv API error: {}", resp.status());
    }

    let body = resp.text().await?;

    let entry = extract_between(&body, "<entry>", "</entry>")
        .ok_or_else(|| anyhow::anyhow!("Paper not found on arXiv: {}", arxiv_id))?;

    let title = extract_between(&entry, "<title>", "</title>")
        .unwrap_or_default()
        .trim()
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let abstract_text = extract_between(&entry, "<summary>", "</summary>")
        .map(|s| s.trim().replace('\n', " ").split_whitespace().collect::<Vec<_>>().join(" "));

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

fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = text.find(start)? + start.len();
    let e = text[s..].find(end)? + s;
    Some(&text[s..e])
}
