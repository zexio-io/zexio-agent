use serde::{Deserialize, Serialize};

const AGENT_API_URL: &str = "http://localhost:8081";

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: f64,
    pub memory: MemoryStats,
    pub storage: StorageStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub used: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TunnelStartRequest {
    pub provider: String,
    pub token: String,
}

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

#[tauri::command]
async fn start_tunnel(provider: String, token: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let request_body = TunnelStartRequest { provider, token };
    
    let response = client
        .post(format!("{}/tunnel/start", AGENT_API_URL))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to start tunnel: {}", e))?;
    
    if response.status().is_success() {
        Ok("Tunnel started successfully".to_string())
    } else {
        let error = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to start tunnel: {}", error))
    }
}

#[tauri::command]
async fn stop_tunnel() -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(format!("{}/tunnel/stop", AGENT_API_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to stop tunnel: {}", e))?;
    
    if response.status().is_success() {
        Ok("Tunnel stopped successfully".to_string())
    } else {
        let error = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to stop tunnel: {}", error))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            health_check,
            get_system_stats,
            start_tunnel,
            stop_tunnel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
