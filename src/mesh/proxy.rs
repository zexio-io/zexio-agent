use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use hyper::client::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use crate::state::AppState;
use tracing::{info, error, debug};

pub async fn mesh_proxy_handler(
    State(state): State<AppState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let host = req.headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    debug!("Mesh Proxy receiving request for host: {}", host);

    // 1. Resolve Host to Project ID & Port
    // Logic:
    // - [uuid].vectis.dev -> port = hash(uuid)
    // - [env]--[uuid].vectis.dev -> port = hash(uuid)
    // - [id].zexio.internal -> port = hash(id)
    
    let project_id = resolve_project_id_from_host(host);
    
    let project_id = match project_id {
        Some(id) => id,
        None => {
            debug!("Host {} not matched to any project", host);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // 2. Deterministic Port Calculation
    let port = 8000 + (crc32fast::hash(project_id.as_bytes()) % 1000) as u16;

    // 3. Proxy the request to localhost:port
    let path_query = req.uri().path_and_query().map(|v| v.as_str()).unwrap_or("/");
    let target_uri = format!("http://127.0.0.1:{}{}", port, path_query);
    
    debug!("Proxying {} to {}", host, target_uri);

    let client: Client<HttpConnector, Body> = Client::builder(TokioExecutor::new()).build_http();
    
    *req.uri_mut() = target_uri.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match client.request(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(e) => {
            error!("Proxy error for {}: {}", host, e);
            Ok((StatusCode::BAD_GATEWAY, format!("Project {} is not reachable on port {}", project_id, port)).into_response())
        }
    }
}

fn resolve_project_id_from_host(host: &str) -> Option<String> {
    // Strip port if exists
    let host = host.split(':').next().unwrap_or(host);

    // Handle .zexio.internal
    if host.ends_with(".zexio.internal") {
        return Some(host.replace(".zexio.internal", ""));
    }

    // Handle .zexio.app (Wildcard)
    // Format: [env]--[uuid].zexio.app atau [uuid].zexio.app
    if host.ends_with(".zexio.app") {
        let prefix = host.replace(".zexio.app", "");
        if let Some(uuid) = prefix.split("--").last() {
            return Some(uuid.to_string());
        }
        return Some(prefix);
    }

    None
}
