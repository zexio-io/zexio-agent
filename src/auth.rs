use axum::{
    async_trait,
    extract::{FromRequest, Request},
    body::Bytes,
    http::{HeaderMap},
};
use crate::state::AppState;
use crate::errors::AppError;
use crate::crypto::Crypto;
use tracing::{warn};

// Extractor for Worker-level authentication (global secret)
pub struct WorkerAuth(pub Bytes);

#[async_trait]
impl<S> FromRequest<S> for WorkerAuth
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = state.clone();
        let secret = &app_state.worker_secret;

        let (parts, body) = req.into_parts();
        let headers = parts.headers;

        let signature = get_signature(&headers)?;
        let bytes = axum::body::to_bytes(body, usize::MAX).await
            .map_err(|e| AppError::BadRequest(format!("Failed to read body: {}", e)))?;

        if !Crypto::verify_signature(secret, &bytes, signature) {
            warn!("Invalid worker signature");
            return Err(AppError::Unauthorized("Invalid signature".to_string()));
        }

        Ok(WorkerAuth(bytes))
    }
}

fn get_signature(headers: &HeaderMap) -> Result<&str, AppError> {
    headers.get("X-Signature")
        .ok_or(AppError::Unauthorized("Missing X-Signature header".into()))?
        .to_str()
        .map_err(|_| AppError::BadRequest("Invalid X-Signature header".into()))
}
