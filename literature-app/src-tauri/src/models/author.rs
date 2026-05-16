use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub first_author_count: i32,
    pub corresponding_author_count: i32,
    pub paper_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub paper_count: i32,
    pub author_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub paper_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithPapers {
    pub author_name: String,
    pub papers: Vec<crate::models::paper::Paper>,
}
