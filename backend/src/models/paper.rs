use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub abstract_text: Option<String>,
    pub user_notes: Option<String>,
    pub year: Option<i32>,
    pub journal: Option<String>,
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_serialization() {
        let paper = Paper {
            id: "p-1".to_string(),
            title: "Test Paper".to_string(),
            doi: Some("10.1234/test".to_string()),
            arxiv_id: None,
            abstract_text: Some("Abstract text".to_string()),
            user_notes: Some("# Notes\n- point 1".to_string()),
            year: Some(2024),
            journal: Some("Nature".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&paper).unwrap();
        assert!(json.contains("p-1"));
        assert!(json.contains("Test Paper"));
        assert!(json.contains("10.1234/test"));

        let deserialized: Paper = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "p-1");
        assert_eq!(deserialized.doi, Some("10.1234/test".to_string()));
        assert_eq!(deserialized.arxiv_id, None);
        assert_eq!(deserialized.year, Some(2024));
    }

    #[test]
    fn test_paper_with_null_fields() {
        let json = r#"{
            "id": "p-2",
            "title": "Minimal Paper",
            "doi": null,
            "arxiv_id": null,
            "abstract_text": null,
            "user_notes": null,
            "year": null,
            "journal": null,
            "created_at": "2025-01-01"
        }"#;
        let paper: Paper = serde_json::from_str(json).unwrap();
        assert_eq!(paper.id, "p-2");
        assert!(paper.doi.is_none());
        assert!(paper.arxiv_id.is_none());
        assert!(paper.abstract_text.is_none());
        assert!(paper.user_notes.is_none());
        assert!(paper.year.is_none());
        assert!(paper.journal.is_none());
    }

    #[test]
    fn test_paper_with_markdown_notes() {
        let paper = Paper {
            id: "p-3".to_string(),
            title: "MD Notes".to_string(),
            doi: None,
            arxiv_id: Some("2301.00001".to_string()),
            abstract_text: None,
            user_notes: Some("# Key Findings\n\n- Result 1\n- Result 2\n\n**Bold text**".to_string()),
            year: Some(2023),
            journal: None,
            created_at: "2025-01-01".to_string(),
        };
        let json = serde_json::to_string(&paper).unwrap();
        let deserialized: Paper = serde_json::from_str(&json).unwrap();
        let notes = deserialized.user_notes.unwrap();
        assert!(notes.contains("# Key Findings"));
        assert!(notes.contains("**Bold text**"));
    }
}
