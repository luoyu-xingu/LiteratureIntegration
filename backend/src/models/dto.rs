use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportPaperRequest {
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaperRequest {
    pub user_notes: Option<String>,
    pub corresponding_author_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaperDetailResponse {
    pub paper: super::paper::Paper,
    pub first_author: Option<super::author::Author>,
    pub corresponding_author: Option<super::author::Author>,
    pub keywords: Vec<super::keyword::Keyword>,
}

#[derive(Debug, Serialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub paper_count: i32,
    pub author_type: String,
}

#[derive(Debug, Serialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub paper_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub format: String,
    pub group_by: Option<String>,
    pub filter: Option<ExportFilter>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ExportFilter {
    pub author_ids: Option<Vec<String>>,
    pub keyword_ids: Option<Vec<String>>,
    pub year_range: Option<(i32, i32)>,
}

#[derive(Debug, Serialize)]
pub struct AuthorWithPapers {
    pub author: super::author::Author,
    pub papers: Vec<super::paper::Paper>,
}
