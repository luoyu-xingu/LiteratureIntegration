use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::models::dto::{ErrorResponse, ErrorBody};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Workspace not found: {0}")]
    WorkspaceNotFound(String),

    #[error("Paper not found: {0}")]
    PaperNotFound(String),

    #[error("Author not found: {0}")]
    AuthorNotFound(String),

    #[error("Import failed: {0}")]
    ImportFailed(String),

    #[error("Neo4j error: {0}")]
    Neo4jError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::WorkspaceNotFound(_) => (StatusCode::NOT_FOUND, "WORKSPACE_NOT_FOUND", self.to_string()),
            AppError::PaperNotFound(_) => (StatusCode::NOT_FOUND, "PAPER_NOT_FOUND", self.to_string()),
            AppError::AuthorNotFound(_) => (StatusCode::NOT_FOUND, "AUTHOR_NOT_FOUND", self.to_string()),
            AppError::ImportFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "IMPORT_FAILED", self.to_string()),
            AppError::Neo4jError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "NEO4J_ERROR", self.to_string()),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", self.to_string()),
            AppError::ExternalApiError(_) => (StatusCode::BAD_GATEWAY, "EXTERNAL_API_ERROR", self.to_string()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error".into()),
        };
        // 使用类型化结构体直接序列化，避免 serde_json::json! 构造中间 Value 树
        // (Map + 两个 String 键) 再二次序列化的开销。输出字节与原实现完全一致。
        let body = ErrorResponse {
            error: ErrorBody { code, message },
        };
        (status, axum::Json(body)).into_response()
    }
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        AppError::Neo4jError(err.to_string())
    }
}

impl From<neo4rs::DeError> for AppError {
    fn from(err: neo4rs::DeError) -> Self {
        AppError::Neo4jError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_not_found_display() {
        let err = AppError::WorkspaceNotFound("ws-123".to_string());
        let msg = err.to_string();
        assert!(msg.contains("ws-123"));
        assert!(msg.contains("Workspace not found"));
    }

    #[test]
    fn test_paper_not_found_display() {
        let err = AppError::PaperNotFound("paper-456".to_string());
        let msg = err.to_string();
        assert!(msg.contains("paper-456"));
        assert!(msg.contains("Paper not found"));
    }

    #[test]
    fn test_author_not_found_display() {
        let err = AppError::AuthorNotFound("author-789".to_string());
        let msg = err.to_string();
        assert!(msg.contains("author-789"));
    }

    #[test]
    fn test_import_failed_display() {
        let err = AppError::ImportFailed("bad doi".to_string());
        let msg = err.to_string();
        assert!(msg.contains("bad doi"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = AppError::ValidationError("missing field".to_string());
        let msg = err.to_string();
        assert!(msg.contains("missing field"));
    }

    #[test]
    fn test_external_api_error_display() {
        let err = AppError::ExternalApiError("timeout".to_string());
        let msg = err.to_string();
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_neo4j_error_display() {
        let err = AppError::Neo4jError("connection refused".to_string());
        let msg = err.to_string();
        assert!(msg.contains("connection refused"));
    }

    #[tokio::test]
    async fn test_error_into_response_workspace_not_found() {
        let err = AppError::WorkspaceNotFound("ws-1".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_error_into_response_paper_not_found() {
        let err = AppError::PaperNotFound("p-1".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_error_into_response_validation() {
        let err = AppError::ValidationError("bad input".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_error_into_response_import_failed() {
        let err = AppError::ImportFailed("doi not found".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_error_into_response_neo4j() {
        let err = AppError::Neo4jError("db error".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_error_into_response_external_api() {
        let err = AppError::ExternalApiError("api down".into());
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
    }
}
