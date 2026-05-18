pub mod crossref;
pub mod arxiv;

use crate::models::paper::Paper;

pub struct ImportResult {
    pub paper: Paper,
    pub abstract_text: Option<String>,
}

pub async fn import_by_identifier(identifier: &str) -> anyhow::Result<ImportResult> {
    let identifier = identifier.trim();
    if identifier.contains('/') && identifier.contains("10.") {
        crossref::fetch_by_doi(identifier).await
    } else if identifier.starts_with("http") && identifier.contains("arxiv.org") {
        let id = identifier.split('/').last().unwrap_or(identifier);
        arxiv::fetch_by_arxiv_id(id).await
    } else {
        arxiv::fetch_by_arxiv_id(identifier).await
    }
}
