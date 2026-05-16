use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_serialization() {
        let ws = Workspace {
            id: "test-id".to_string(),
            name: "Test Workspace".to_string(),
            description: "A test".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&ws).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("Test Workspace"));

        let deserialized: Workspace = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test-id");
        assert_eq!(deserialized.name, "Test Workspace");
        assert_eq!(deserialized.description, "A test");
    }

    #[test]
    fn test_workspace_deserialization() {
        let json = r#"{"id":"ws-1","name":"My WS","description":"desc","created_at":"2025-06-01"}"#;
        let ws: Workspace = serde_json::from_str(json).unwrap();
        assert_eq!(ws.id, "ws-1");
        assert_eq!(ws.name, "My WS");
    }

    #[test]
    fn test_workspace_clone() {
        let ws = Workspace {
            id: "c1".to_string(),
            name: "Clone".to_string(),
            description: "test".to_string(),
            created_at: "2025".to_string(),
        };
        let cloned = ws.clone();
        assert_eq!(cloned.id, ws.id);
        assert_eq!(cloned.name, ws.name);
    }
}
