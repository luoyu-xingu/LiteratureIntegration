use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

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
        let body = json!({ "error": { "code": code, "message": message } });
        (status, axum::Json(body)).into_response()
    }
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        AppError::Neo4jError(err.to_string())
    }
}
