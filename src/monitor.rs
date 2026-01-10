use axum::{
    extract::{Path, State},
    Json,
    response::sse::{Event, KeepAlive, Sse},
    http::StatusCode,
};
use serde::Serialize;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use crate::{state::AppState, auth::WorkerAuth, errors::AppError};
use std::process::Command;
use std::time::Duration;
use futures::stream::{self, Stream};
use std::convert::Infallible;

#[derive(Serialize)]
pub struct SystemStats {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    uptime: u64,
}

pub async fn global_stats_handler(
    State(_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Unfold allows us to maintain the `System` state across the stream
    let stream = stream::unfold(
        System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        ), 
        |mut sys| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            sys.refresh_cpu();
            sys.refresh_memory();
            
            let stats = SystemStats {
                cpu_usage: sys.global_cpu_info().cpu_usage(),
                memory_used: sys.used_memory(),
                memory_total: sys.total_memory(),
                uptime: System::uptime(),
            };

            let json = serde_json::to_string(&stats).unwrap_or_default();
            
            // Yield event and pass updated state (sys) back
            Some((Ok(Event::default().data(json)), sys))
        }
    );

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Serialize)]
pub struct ProjectStatus {
    id: String,
    active: bool,
    pid: Option<u32>,
    status_line: String,
}

pub async fn project_monitor_handler(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
    WorkerAuth(_): WorkerAuth,
) -> Result<Json<ProjectStatus>, AppError> {
    // Check systemd status
    // systemctl is-active app@{id}
    
    let service_name = format!("app@{}", project_id);
    let output = Command::new("systemctl")
        .arg("status")
        .arg(&service_name)
        .output()
        .map_err(|_| AppError::InternalServerError)?;

    let status_str = String::from_utf8_lossy(&output.stdout);
    let active = status_str.contains("Active: active (running)");
    
    // Parse PID roughly?
    // "Main PID: 1234 (app)"
    let pid = status_str.lines()
        .find(|l| l.trim().starts_with("Main PID:"))
        .and_then(|l| l.split_whitespace().nth(2))
        .and_then(|p| p.parse::<u32>().ok());

    Ok(Json(ProjectStatus {
        id: project_id,
        active,
        pid,
        status_line: status_str.lines().next().unwrap_or("Unknown").to_string(),
    }))
}
