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

#[derive(Debug, Serialize, Deserialize)]
pub struct PaperDetailResponse {
    pub paper: super::paper::Paper,
    pub first_author: Option<super::author::Author>,
    pub corresponding_author: Option<super::author::Author>,
    pub keywords: Vec<super::keyword::Keyword>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorWithPapers {
    pub author: super::author::Author,
    pub papers: Vec<super::paper::Paper>,
}

/// Typed search response. Serialized directly via serde instead of building a
/// `serde_json::Value` tree (which `serde_json::json!` does), avoiding a full
/// intermediate Value allocation + re-serialization pass.
#[derive(Debug, Serialize)]
#[serde(tag = "mode")]
pub enum SearchResponse {
    #[serde(rename = "keyword")]
    Keyword {
        query: String,
        results: Vec<super::paper::Paper>,
    },
    #[serde(rename = "author")]
    Author {
        query: String,
        results: Vec<AuthorWithPapers>,
    },
}

/// Typed response for the "remove paper from workspace" endpoint.
///
/// Serializing this struct directly via serde avoids the intermediate
/// `serde_json::Value` tree that `serde_json::json!` would build (one Map +
/// one Bool allocation) and then re-serialize, saving an allocation plus a
/// second serialization pass on every delete request. The JSON field name
/// (`removed`) is part of the public API contract with the frontend.
#[derive(Debug, Serialize)]
pub struct PaperRemovedResponse {
    pub removed: bool,
}

/// Typed response for the "delete workspace" endpoint. Same rationale as
/// `PaperRemovedResponse`: avoid the `serde_json::Value` round-trip. The JSON
/// field name (`deleted`) is part of the public API contract with the frontend.
#[derive(Debug, Serialize)]
pub struct WorkspaceDeletedResponse {
    pub deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_workspace_request_deserialization() {
        let json = r#"{"name":"Test","description":"desc"}"#;
        let req: CreateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test");
        assert_eq!(req.description, Some("desc".to_string()));
    }

    #[test]
    fn test_create_workspace_request_no_description() {
        let json = r#"{"name":"Test"}"#;
        let req: CreateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test");
        assert!(req.description.is_none());
    }

    #[test]
    fn test_update_workspace_request_partial() {
        let json = r#"{"name":"Updated"}"#;
        let req: UpdateWorkspaceRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("Updated".to_string()));
        assert!(req.description.is_none());
    }

    #[test]
    fn test_import_paper_request() {
        let json = r#"{"identifier":"10.1234/test"}"#;
        let req: ImportPaperRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.identifier, "10.1234/test");
    }

    #[test]
    fn test_update_paper_request_notes() {
        let json = r#"{"user_notes":"My Notes - Point 1"}"#;
        let req: UpdatePaperRequest = serde_json::from_str(json).unwrap();
        assert!(req.user_notes.unwrap().contains("My Notes"));
    }

    #[test]
    fn test_export_request_defaults() {
        let json = r#"{"format":"markdown"}"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.format, "markdown");
        assert!(req.group_by.is_none());
        assert!(req.filter.is_none());
    }

    #[test]
    fn test_export_filter_defaults() {
        let filter = ExportFilter::default();
        assert!(filter.author_ids.is_none());
        assert!(filter.keyword_ids.is_none());
        assert!(filter.year_range.is_none());
    }

    #[test]
    fn test_paper_detail_response_serialization() {
        let resp = PaperDetailResponse {
            paper: crate::models::paper::Paper {
                id: "p-1".into(),
                title: "Test".into(),
                doi: None,
                arxiv_id: None,
                abstract_text: None,
                user_notes: None,
                year: Some(2024),
                journal: None,
                created_at: "2025".into(),
            },
            first_author: None,
            corresponding_author: None,
            keywords: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("p-1"));
    }

    #[test]
    fn test_graph_data_response_serialization() {
        let resp = GraphDataResponse {
            nodes: vec![GraphNode {
                id: "n-1".into(),
                name: "Author1".into(),
                paper_count: 3,
                author_type: "first".into(),
            }],
            links: vec![GraphLink {
                source: "n-1".into(),
                target: "n-2".into(),
                paper_count: 2,
            }],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("Author1"));
        assert!(json.contains("n-2"));
    }

    #[test]
    fn test_author_with_papers_serialization() {
        let awp = AuthorWithPapers {
            author: crate::models::author::Author {
                id: "a-1".into(),
                name: "Author".into(),
                orcid: None,
            },
            papers: vec![],
        };
        let json = serde_json::to_string(&awp).unwrap();
        assert!(json.contains("Author"));
    }

    #[test]
    fn test_paper_removed_response_serialization() {
        let json = serde_json::to_string(&PaperRemovedResponse { removed: true }).unwrap();
        assert_eq!(json, r#"{"removed":true}"#);
    }

    #[test]
    fn test_workspace_deleted_response_serialization() {
        let json = serde_json::to_string(&WorkspaceDeletedResponse { deleted: true }).unwrap();
        assert_eq!(json, r#"{"deleted":true}"#);
    }
}
