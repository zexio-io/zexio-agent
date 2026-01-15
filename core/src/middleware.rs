use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::state::AppState;

/// Smart authentication middleware that:
/// - Skips auth in standalone mode (local development)
/// - Requires auth in cloud mode (production)
pub async fn smart_auth_middleware<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Check if we're in cloud mode
    let is_cloud_mode = state.settings.cloud.token.is_some() 
        && state.settings.cloud.worker_id.is_some();
    
    // Standalone mode: no auth required
    if !is_cloud_mode {
        return Ok(next.run(request).await);
    }
    
    // Cloud mode: require authentication
    worker_auth_middleware(State(state), request, next).await
}

/// Original worker authentication (for cloud mode)
pub async fn worker_auth_middleware<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract signature from header
    let signature = request
        .headers()
        .get("x-zexio-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Get request body for verification (simplified - in real impl, clone body)
    // For now, we'll just check if signature exists
    // In production, verify HMAC signature of request body
    
    if signature.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // TODO: Implement proper HMAC verification
    // let body_bytes = ...; // extract body
    // if !Crypto::verify_signature(&state.worker_secret, &body_bytes, signature) {
    //     return Err(StatusCode::UNAUTHORIZED);
    // }

    Ok(next.run(request).await)
}
