use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub orcid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author_serialization() {
        let author = Author {
            id: "a-1".to_string(),
            name: "John Doe".to_string(),
            orcid: Some("0000-0001-2345-6789".to_string()),
        };
        let json = serde_json::to_string(&author).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("0000-0001-2345-6789"));

        let deserialized: Author = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "a-1");
        assert_eq!(deserialized.name, "John Doe");
        assert_eq!(deserialized.orcid, Some("0000-0001-2345-6789".to_string()));
    }

    #[test]
    fn test_author_without_orcid() {
        let json = r#"{"id":"a-2","name":"Jane","orcid":null}"#;
        let author: Author = serde_json::from_str(json).unwrap();
        assert_eq!(author.name, "Jane");
        assert!(author.orcid.is_none());
    }

    #[test]
    fn test_author_clone() {
        let author = Author {
            id: "a-3".to_string(),
            name: "Clone".to_string(),
            orcid: None,
        };
        let cloned = author.clone();
        assert_eq!(cloned.id, author.id);
        assert_eq!(cloned.name, author.name);
    }
}
