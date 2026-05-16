use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_serialization() {
        let kw = Keyword {
            id: "k-1".to_string(),
            name: "deep learning".to_string(),
        };
        let json = serde_json::to_string(&kw).unwrap();
        assert!(json.contains("deep learning"));

        let deserialized: Keyword = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "k-1");
        assert_eq!(deserialized.name, "deep learning");
    }

    #[test]
    fn test_keyword_deserialization() {
        let json = r#"{"id":"k-2","name":"transformer"}"#;
        let kw: Keyword = serde_json::from_str(json).unwrap();
        assert_eq!(kw.name, "transformer");
    }
}
