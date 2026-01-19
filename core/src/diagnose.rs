use crate::config::Settings;
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn run_diagnostics(settings: &Settings) -> anyhow::Result<()> {
    println!(
        "\n{}",
        "🔍 Starting Zexio Agent Diagnostics...".bold().cyan()
    );
    println!("{}", "=".repeat(50).dimmed());

    // 1. Connectivity Check
    check_connectivity(settings).await;

    // 2. Identity Check
    check_identity(settings);

    // 3. Service/Daemon Check
    check_service();

    // 4. Mesh Proxy Check
    check_mesh(settings);

    println!("\n{}", "=".repeat(50).dimmed());
    println!("{}", "✅ Diagnostics Complete.".bold().green());
    Ok(())
}

pub async fn run_info(settings: &Settings) -> anyhow::Result<()> {
    println!("\n{}", "ℹ️  Zexio Agent Information".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());

    // Identity Info
    let identity_path = &settings.secrets.identity_path;
    if Path::new(identity_path).exists() {
        if let Ok(content) = fs::read_to_string(identity_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                println!(
                    "{:<15} : {}",
                    "Node ID".bold(),
                    json["worker_id"].as_str().unwrap_or("Unknown")
                );
                println!(
                    "{:<15} : {}",
                    "Node Name".bold(),
                    json["worker_name"].as_str().unwrap_or("Unknown")
                );
            }
        }
    } else {
        println!("{}", "⚠️  Not registered (identity.json missing)".yellow());
    }

    // Version
    println!(
        "{:<15} : {}",
        "Agent Version".bold(),
        env!("CARGO_PKG_VERSION")
    );

    // Endpoints
    println!("\n{}", "📡 Endpoints:".bold());
    println!("  {:<12} : {}", "Cloud API", settings.cloud.api_url);

    // Relay Server URL (Used by tunnel)
    let relay_url =
        std::env::var("RELAY_URL").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    println!("  {:<12} : {}", "Relay Server", relay_url.green());

    println!(
        "  {:<12} : http://{}:{}",
        "Mgmt API", settings.server.host, settings.server.port
    );
    println!(
        "  {:<12} : port {}",
        "Mesh Proxy", settings.server.mesh_port
    );

    println!("\n{}", "=".repeat(50).dimmed());
    Ok(())
}

async fn check_connectivity(settings: &Settings) {
    print!("{:<35} ", "Checking Cloud API Connection...");
    let client = reqwest::Client::new();
    let health_url = format!("{}/health", settings.cloud.api_url);

    match client
        .get(&health_url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(res) if res.status().is_success() => println!("{}", "[ OK ]".bold().green()),
        Ok(res) => println!("{} (Status: {})", "[ FAIL ]".bold().red(), res.status()),
        Err(e) => println!("{} ({})", "[ FAIL ]".bold().red(), e),
    }

    print!("{:<35} ", "Checking Relay Connectivity...");
    let relay_url =
        std::env::var("RELAY_URL").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    // Simple TCP connect check for relay (usually gRPC)
    if let Ok(url) = url::Url::parse(&relay_url) {
        if let Some(host) = url.host_str() {
            let port = url.port_or_known_default().unwrap_or(80);
            // Use tokio's TCP connect for non-blocking if needed, but std is fine for diag
            match std::net::TcpStream::connect_timeout(
                &format!("{}:{}", host, port)
                    .parse()
                    .unwrap_or("127.0.0.1:50051".parse().unwrap()),
                std::time::Duration::from_secs(3),
            ) {
                Ok(_) => println!("{}", "[ OK ]".bold().green()),
                Err(e) => println!("{} ({})", "[ FAIL ]".bold().red(), e),
            }
        }
    }
}

fn check_identity(settings: &Settings) {
    print!("{:<35} ", "Verifying Local Identity...");
    if Path::new(&settings.secrets.identity_path).exists() {
        println!("{}", "[ OK ]".bold().green());
    } else {
        println!("{}", "[ MISSING ]".bold().yellow());
        println!("  (Run 'zexio login' or 'zexio connect <token>')");
    }
}

fn check_service() {
    print!("{:<35} ", "Checking Background Service...");

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("systemctl")
            .arg("is-active")
            .arg("zexio")
            .output();
        if let Ok(out) = output {
            if String::from_utf8_lossy(&out.stdout).trim() == "active" {
                println!("{}", "[ RUNNING ]".bold().green());
            } else {
                println!("{}", "[ INACTIVE ]".bold().yellow());
            }
        } else {
            println!("{}", "[ NOT FOUND ]".bold().red());
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("launchctl")
            .arg("list")
            .arg("io.zexio.agent")
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                println!("{}", "[ RUNNING ]".bold().green());
            } else {
                println!("{}", "[ INACTIVE/NOT FOUND ]".bold().yellow());
            }
        } else {
            println!("{}", "[ ERROR ]".bold().red());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("sc").arg("query").arg("ZexioAgent").output();
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("RUNNING") {
                println!("{}", "[ RUNNING ]".bold().green());
            } else {
                println!("{}", "[ STOPPED/NOT FOUND ]".bold().yellow());
            }
        } else {
            println!("{}", "[ ERROR ]".bold().red());
        }
    }
}

fn check_mesh(settings: &Settings) {
    print!("{:<35} ", "Checking Mesh Proxy Port...");
    match std::net::TcpListener::bind(format!("0.0.0.0:{}", settings.server.mesh_port)) {
        Ok(_) => {
            println!("{}", "[ READY ]".bold().blue());
            println!("  (Proxy port is available)");
        }
        Err(_) => {
            println!("{}", "[ IN USE ]".bold().green());
            println!("  (Proxy port is occupied, likely by a running agent)");
        }
    }
}
