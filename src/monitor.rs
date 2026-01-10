use axum::{
    extract::{Path, State},
    Json,
    response::sse::{Event, KeepAlive, Sse},
};
use serde::Serialize;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use crate::{state::AppState, errors::AppError};
use std::process::Command;
use std::time::Duration;
use futures::stream::Stream;
use std::convert::Infallible;

#[derive(Serialize)]
pub struct SystemStats {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    memory_percent: f32,
    disk_used: u64,
    disk_total: u64,
    disk_percent: f32,
}

// JSON endpoint (one-time)
pub async fn global_stats_handler(
    State(_state): State<AppState>,
) -> Result<Json<SystemStats>, AppError> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    // Get disk stats (root partition)
    sys.refresh_disks_list();
    let (disk_used, disk_total) = sys.disks()
        .iter()
        .find(|disk| disk.mount_point().to_str() == Some("/"))
        .map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            (used, total)
        })
        .unwrap_or((0, 0));
    
    let disk_percent = if disk_total > 0 {
        (disk_used as f32 / disk_total as f32) * 100.0
    } else {
        0.0
    };

    Ok(Json(SystemStats {
        cpu_usage,
        memory_used,
        memory_total,
        memory_percent,
        disk_used,
        disk_total,
        disk_percent,
    }))
}

// SSE endpoint (real-time updates every 2 seconds)
pub async fn global_stats_stream(
    State(_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        loop {
            let mut sys = System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );

            sys.refresh_cpu_all();
            tokio::time::sleep(Duration::from_millis(200)).await;
            sys.refresh_cpu_all();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage();
            let memory_used = sys.used_memory();
            let memory_total = sys.total_memory();
            let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

            // Get disk stats (root partition)
            sys.refresh_disks_list();
            let (disk_used, disk_total) = sys.disks()
                .iter()
                .find(|disk| disk.mount_point().to_str() == Some("/"))
                .map(|disk| {
                    let total = disk.total_space();
                    let available = disk.available_space();
                    let used = total - available;
                    (used, total)
                })
                .unwrap_or((0, 0));
            
            let disk_percent = if disk_total > 0 {
                (disk_used as f32 / disk_total as f32) * 100.0
            } else {
                0.0
            };

            let stats = SystemStats {
                cpu_usage,
                memory_used,
                memory_total,
                memory_percent,
                disk_used,
                disk_total,
                disk_percent,
            };

            if let Ok(json) = serde_json::to_string(&stats) {
                yield Ok(Event::default().data(json));
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}


#[derive(Serialize)]
pub struct ProjectStatus {
    status: String,
    active: bool,
}

// JSON endpoint (one-time)
pub async fn project_monitor_handler(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectStatus>, AppError> {
    let unit_name = format!("app@{}.service", project_id);
    
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg(&unit_name)
        .output()
        .map_err(|_| AppError::InternalServerError)?;

        .and_then(|p| p.parse::<u32>().ok());

    Ok(Json(ProjectStatus {
        id: project_id,
        active,
        pid,
        status_line: status_str.lines().next().unwrap_or("Unknown").to_string(),
    }))
}
