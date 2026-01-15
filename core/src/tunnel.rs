use crate::state::AppState;
use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use tracing::{info, error};

// Global static to hold the specific tunnel process? 
// Better to put in AppState, but Child is not Clone/Send easily in async state if not wrapped tightly.
// For MVP, we use a lazy_static or a simple Mutex in a new struct.

pub struct TunnelManager {
    // K: Provider, V: Child Process
    active_tunnels: Mutex<Option<(String, Child)>>, 
}

impl TunnelManager {
    pub fn new() -> Self {
        Self {
            active_tunnels: Mutex::new(None),
        }
    }
}

#[derive(Deserialize)]
pub struct StartTunnelRequest {
    pub provider: String, // "cloudflare" | "pangolin"
    pub token: String,    // Auth token
    pub local_port: Option<u16>, 
}

#[derive(Serialize)]
pub struct TunnelResponse {
    pub status: String,
    pub message: String,
}

pub async fn start_tunnel_handler(
    State(state): State<AppState>,
    Json(payload): Json<StartTunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    
    // 1. Check if tunnel already running
    // Note: accessing AppState -> TunnelManager requires us to add TunnelManager to AppState first.
    // For now, assuming we will add `pub tunnel_manager: Arc<TunnelManager>` to AppState.
    
    let mut manager = state.tunnel_manager.active_tunnels.lock().map_err(|_| 
        (StatusCode::INTERNAL_SERVER_ERROR, "Lock failed".to_string())
    )?;

    if let Some((current_provider, _)) = manager.as_ref() {
        return Err((StatusCode::CONFLICT, format!("Tunnel already running with provider: {}", current_provider)));
    }

    let port = payload.local_port.unwrap_or(state.settings.server.mesh_port);

    info!("Starting tunnel via {} for port {}", payload.provider, port);

    let child = match payload.provider.as_str() {
        "cloudflare" => {
            Command::new("cloudflared")
                .arg("tunnel")
                .arg("run")
                .arg("--token")
                .arg(&payload.token)
                .spawn()
        },
        "pangolin" => {
            // Hypothetical CLI for Pangolin
            Command::new("pangolin")
                .arg("tunnel")
                .arg("--port")
                .arg(port.to_string())
                .arg("--token")
                .arg(&payload.token)
                .spawn()
        },
        _ => return Err((StatusCode::BAD_REQUEST, "Unsupported provider".to_string()))
    };

    match child {
        Ok(process) => {
            *manager = Some((payload.provider.clone(), process));
            Ok(Json(TunnelResponse {
                status: "success".to_string(),
                message: format!("Started {} tunnel forwarding to port {}", payload.provider, port)
            }))
        },
        Err(e) => {
            error!("Failed to start tunnel: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to spawn process: {}", e)))
        }
    }
}

pub async fn stop_tunnel_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut manager = state.tunnel_manager.active_tunnels.lock().map_err(|_| 
        (StatusCode::INTERNAL_SERVER_ERROR, "Lock failed".to_string())
    )?;

    if let Some((provider, mut child)) = manager.take() {
        info!("Stopping {} tunnel...", provider);
        match child.kill() {
            Ok(_) => Ok(Json(TunnelResponse {
                status: "success".to_string(),
                message: format!("Stopped {} tunnel", provider)
            })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to kill process: {}", e)))
        }
    } else {
        Err((StatusCode::NOT_FOUND, "No active tunnel found".to_string()))
    }
}
