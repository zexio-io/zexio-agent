mod auth;
mod caddy;
mod config;
mod crypto;
mod db;
mod deploy;
mod project;
mod services;
mod monitor;
mod streams;
mod errors;
mod server;
mod state;

use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let settings = config::Settings::new()?;
    info!("Starting Plane worker on port {}", settings.server.port);

    // TODO: Initialize DB connection
    // TODO: Initialize AppState

    // Start server
    server::start(settings).await?;

    Ok(())
}
