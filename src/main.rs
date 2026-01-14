mod config;
mod crypto;
mod storage;
mod deploy;
mod project;
mod services;
mod monitor;
mod streams;
mod errors;
mod middleware;
mod mesh;
mod server;
mod state;
mod registration;


use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Check for CLI arguments
    let args: Vec<String> = std::env::args().collect();

    // Load configuration
    let settings = config::Settings::new()?;

    // Handle Commands
    if args.len() > 1 && args[1] == "unregister" {
        registration::unregister(&settings).await?;
        return Ok(());
    }

    info!("Starting Zexio Agent on port {}", settings.server.port);

    // Auto-Registration Handshake
    if let Err(e) = registration::handshake(&settings).await {
        error!("Handshake failed: {}", e);
        // We continue effectively, maybe manual provision is possible? 
        // Or we should exit? For now log and continue.
    }

    // Start server
    server::start(settings).await?;

    Ok(())
}
