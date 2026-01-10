use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::Config(e) => {
                tracing::error!("Config error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration Error")
            }
            AppError::Io(e) => {
                tracing::error!("IO error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "IO Error")
            }
            AppError::Anyhow(e) => {
                tracing::error!("Internal error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
