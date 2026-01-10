use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::{state::AppState, crypto::Crypto};
use tracing::warn;

pub async fn worker_auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    
    // Get signature from header
    let signature = headers
        .get("X-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Get body for verification
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify signature
    if !Crypto::verify_signature(&state.worker_secret, &bytes, signature) {
        warn!("Invalid worker signature");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Reconstruct request with body
    let request = Request::from_parts(parts, Body::from(bytes));
    
    Ok(next.run(request).await)
}
