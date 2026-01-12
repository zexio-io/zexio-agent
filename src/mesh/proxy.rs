use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use hyper::client::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use crate::state::AppState;
use tracing::{info, error, debug};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use redis::AsyncCommands;

#[derive(Debug, Serialize, Deserialize)]
struct ServiceTokenClaims {
    userId: String,
    orgId: String,
    sourceService: String,
    targetService: String,
    workerId: Option<String>,
}

pub async fn mesh_proxy_handler(
    State(state): State<AppState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let host = req.headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    debug!("Mesh Proxy receiving request for host: {}", host);

    // --- 1. Resolve Host ---
    let (target_host, target_port, target_org_id) = match resolve_mesh_dns(&state, host).await {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    // --- 2. Authenticaton Validation ---
    // Extract JWT from Authorization header
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let is_authorized = if let Some(auth_str) = auth_header {
        if let Some(token) = auth_str.strip_prefix("Bearer ") {
            let mut validation = Validation::new(Algorithm::HS256);
            validation.set_issuer(&["zexio-service-mesh"]);
            
            let decoding_key = DecodingKey::from_secret(state.mesh_jwt_secret.as_bytes());
            
            match decode::<ServiceTokenClaims>(token, &decoding_key, &validation) {
                Ok(token_data) => {
                    let token_org_id = token_data.claims.orgId;
                    debug!("Validated mesh token for org: {}", token_org_id);
                    
                    // CRITICAL: Strict Tenant Isolation Check
                    // Only allow if token org matches target service org
                    token_org_id == target_org_id
                },
                Err(e) => {
                    error!("Mesh token validation failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    } else {
        false
    };

    if !is_authorized {
        error!("Tenant Violation: Unauthorized access attempt to {} (Target Org: {})", host, target_org_id);
        return Err(StatusCode::FORBIDDEN);
    }

    // --- 3. Proxy the request ---
    let path_query = req.uri().path_and_query().map(|v| v.as_str()).unwrap_or("/");
    let target_uri = format!("http://{}:{}{}", target_host, target_port, path_query);
    
    debug!("Proxying {} to {}", host, target_uri);

    let client: Client<HttpConnector, Body> = Client::builder(TokioExecutor::new()).build_http();
    
    *req.uri_mut() = target_uri.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match client.request(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(e) => {
            error!("Proxy error for {}: {}", host, e);
            Ok((StatusCode::BAD_GATEWAY, format!("Target {} is not reachable", host)).into_response())
        }
    }
}

async fn resolve_mesh_dns(state: &AppState, host: &str) -> Result<(String, u16, String), ()> {
    let host = host.split(':').next().unwrap_or(host);

    // Format: {userId}.{serviceSlug}.zexio.internal
    if host.ends_with(".zexio.internal") {
        let parts: Vec<&str> = host.split('.').collect();
        
        // If it's a namespaced service DNS (3 parts before zexio.internal)
        if parts.len() >= 4 {
            // Check Redis for global mapping
            let mut conn = state.redis.get_async_connection().await.map_err(|_| ())?;
            let redis_key = format!("service:{}", host);
            let service_info: std::collections::HashMap<String, String> = conn.hgetall(redis_key).await.map_err(|_| ())?;

            if let Some(target_ip) = service_info.get("worker_ip") {
                let port = service_info.get("port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(80);
                
                let owner_id = service_info.get("owner_id").cloned().unwrap_or_default();
                
                // If it's on this worker, use localhost
                let is_on_this_worker = match &state.settings.server.public_ip {
                    Some(ip) => target_ip == ip,
                    None => target_ip == "127.0.0.1" || target_ip == "localhost",
                };

                if is_on_this_worker {
                     return Ok(("127.0.0.1".to_string(), port, owner_id));
                }
                
                return Ok((target_ip.clone(), port, owner_id));
            }
        }

        // Legacy format: [project-id].zexio.internal
        let project_id = host.replace(".zexio.internal", "");
        let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;
        return Ok(("127.0.0.1".to_string(), port, "".to_string())); // Default owner_id for legacy
    }

    // Handle .zexio.app (Wildcard)
    if host.ends_with(".zexio.app") {
        let prefix = host.replace(".zexio.app", "");
        let project_id = if let Some(uuid) = prefix.split("--").last() {
            uuid.to_string()
        } else {
            prefix
        };
        let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;
        return Ok(("127.0.0.1".to_string(), port));
    }

    Err(())
}
