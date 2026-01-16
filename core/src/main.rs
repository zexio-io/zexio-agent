mod config;
mod crypto;
mod deploy;
mod errors;
mod mesh;
mod middleware;
mod monitor;
mod project;
mod registration;
mod server;
mod services;
mod state;
mod storage;
mod streams;

use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging with better formatting
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .init();

    // Print startup banner
    print_banner();

    // Check for CLI arguments
    let args: Vec<String> = std::env::args().collect();

    // Load configuration
    let settings = config::Settings::new()?;

    info!("🔧 Configuration loaded");
    info!(
        "   Management API: http://{}:{}",
        settings.server.host, settings.server.port
    );
    info!("   Mesh Proxy: port {}", settings.server.mesh_port);

    // Handle Commands
    if args.len() > 1 && args[1] == "unregister" {
        info!("🔓 Unregistering from Zexio Cloud...");
        registration::unregister(&settings).await?;
        return Ok(());
    }

    // Auto-Registration Handshake
    if let Err(e) = registration::handshake(&settings).await {
        error!("⚠️  Handshake failed: {}", e);
        info!("   Continuing in standalone mode...");
    }

    info!("🚀 Starting Zexio Agent...");

    // Start server
    server::start(settings).await?;

    Ok(())
}

fn print_banner() {
    println!("\n{}", "=".repeat(60));
    println!("  ____           _         ");
    println!(" |_  / ___ __ _(_) ___    ");
    println!("  / / / -_) _` / / _ \\   ");
    println!(" /___\\___/_,_/_|_\\___/   Agent v0.3.0");
    println!("{}", "=".repeat(60));
    println!("  Deploy Anything. Anywhere. Securely.");
    println!("{}\n", "=".repeat(60));
}
