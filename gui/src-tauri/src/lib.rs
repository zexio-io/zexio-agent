use serde::{Deserialize, Serialize};

const AGENT_API_URL: &str = "http://127.0.0.1:8081";

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_usage: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_percent: f64,
    pub disk_used: u64,
    pub disk_total: u64,
    pub disk_percent: f64,
    pub total_projects: u32,
}

// MemoryStats and StorageStats structs are no longer needed as per ROUTES.md

// MemoryStats and StorageStats structs are no longer needed as per ROUTES.md

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_system_stats() -> Result<SystemStats, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/stats", AGENT_API_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch stats: {}", e))?;

    let stats = response
        .json::<SystemStats>()
        .await
        .map_err(|e| format!("Failed to parse stats: {}", e))?;

    Ok(stats)
}

#[tauri::command]
async fn health_check() -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", AGENT_API_URL))
        .send()
        .await
        .map_err(|e| format!("Agent not running: {}", e))?;

    let status = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(status)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            health_check,
            get_system_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
