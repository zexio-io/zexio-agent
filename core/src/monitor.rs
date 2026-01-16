use crate::{errors::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::process::Command;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

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

    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    // Get disk stats (root partition)
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let (disk_used, disk_total) = disks
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

            sys.refresh_cpu();
            tokio::time::sleep(Duration::from_millis(200)).await;
            sys.refresh_cpu();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let memory_used = sys.used_memory();
            let memory_total = sys.total_memory();
            let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

            // Get disk stats (root partition)
            let disks = sysinfo::Disks::new_with_refreshed_list();
            let (disk_used, disk_total) = disks
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

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let active = status == "active";

    Ok(Json(ProjectStatus { status, active }))
}

// SSE endpoint (real-time updates every 3 seconds)
pub async fn project_monitor_stream(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        let unit_name = format!("app@{}.service", project_id);

        loop {
            let output = Command::new("systemctl")
                .arg("is-active")
                .arg(&unit_name)
                .output();

            if let Ok(output) = output {
                let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let active = status == "active";

                let project_status = ProjectStatus { status, active };

                if let Ok(json) = serde_json::to_string(&project_status) {
                    yield Ok(Event::default().data(json));
                }
            }

            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Serialize)]
pub struct SyncResponse {
    pub status: String,
    pub version: String,
    pub stats: SystemStats,
}

// Manual sync endpoint for dashboard to trigger
pub async fn sync_handler(State(_state): State<AppState>) -> Result<Json<SyncResponse>, AppError> {
    // Reuse global_stats logic
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let (disk_used, disk_total) = disks
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

    Ok(Json(SyncResponse {
        status: "online".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        stats,
    }))
}
#[derive(Deserialize)]
pub struct ConfigureFirewallRequest {
    pub port: u16,
    pub allowed_tenants: Vec<String>,
}

pub async fn configure_firewall_handler(
    State(_state): State<AppState>,
    Json(payload): Json<ConfigureFirewallRequest>,
) -> Result<Json<SyncResponse>, AppError> {
    use crate::mesh::firewall::FirewallManager;

    FirewallManager::update_rules(payload.port, &payload.allowed_tenants)
        .map_err(|_| AppError::InternalServerError)?;

    // Return dummy sync response for now
    Ok(Json(SyncResponse {
        status: "firewall_updated".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        stats: get_dummy_stats(), // I should extract global_stats logic to a helper
    }))
}

fn get_dummy_stats() -> SystemStats {
    SystemStats {
        cpu_usage: 0.0,
        memory_used: 0,
        memory_total: 0,
        memory_percent: 0.0,
        disk_used: 0,
        disk_total: 0,
        disk_percent: 0.0,
    }
}
