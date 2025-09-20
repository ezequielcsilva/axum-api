use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found"),
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
            ApiError::Anyhow(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}