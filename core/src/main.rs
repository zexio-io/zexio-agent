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

use clap::{Parser, Subcommand};
use tracing::{error, info};

/// Zexio Agent - Deploy Anything. Anywhere. Securely.
#[derive(Parser)]
#[command(name = "zexio")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a tunnel to expose a local port to the internet
    Up {
        /// Local port to expose (e.g., 3000)
        port: u16,
    },
    /// Unregister this agent from Zexio Cloud
    Unregister,
}

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

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let settings = config::Settings::new()?;

    info!("🔧 Configuration loaded");
    info!(
        "   Management API: http://{}:{}",
        settings.server.host, settings.server.port
    );
    info!("   Mesh Proxy: port {}", settings.server.mesh_port);

    // Handle Commands
    match cli.command {
        Some(Commands::Unregister) => {
            info!("🔓 Unregistering from Zexio Cloud...");
            registration::unregister(&settings).await?;
            return Ok(());
        }
        Some(Commands::Up { port }) => {
            // Auto-Registration Handshake
            if let Err(e) = registration::handshake(&settings).await {
                error!("⚠️  Handshake failed: {}", e);
                info!("   Continuing in standalone mode...");
            }

            info!("🚀 Starting Zexio Agent with Tunnel...");
            info!("🎯 Exposing local port {} to the internet", port);

            // Start server with tunnel
            server::start(settings, Some(port)).await?;
        }
        None => {
            // No subcommand - run management API only
            info!("🚀 Starting Zexio Agent (Management API only)...");
            info!("💡 TIP: Use 'zexio up <port>' to start a tunnel");

            // Auto-Registration Handshake
            if let Err(e) = registration::handshake(&settings).await {
                error!("⚠️  Handshake failed: {}", e);
                info!("   Continuing in standalone mode...");
            }

            // Start server without tunnel
            server::start(settings, None).await?;
        }
    }

    Ok(())
}

fn print_banner() {
    println!("\n{}", "=".repeat(60));
    println!("  ____           _         ");
    println!(" |_  / ___ __ _(_) ___    ");
    println!("  / / / -_) _` / / _ \\   ");
    println!(" /___\\___/_,_/_|_\\___/   Agent v0.3.1");
    println!("{}", "=".repeat(60));
    println!("  Deploy Anything. Anywhere. Securely.");
    println!("{}\n", "=".repeat(60));
}
