use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub first_author: Option<String>,
    pub corresponding_author: Option<String>,
    pub keywords: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperFile {
    pub frontmatter: Paper,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperDetailResponse {
    pub paper: Paper,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportPaperRequest {
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaperRequest {
    pub user_notes: Option<String>,
}
