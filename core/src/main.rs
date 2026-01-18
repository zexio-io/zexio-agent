mod config;
mod crypto;
mod daemon;
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
    /// Authenticate with Zexio Cloud
    Login,
    /// Connect to Zexio Cloud using a provisioning token
    Connect {
        /// Provisioning token from the dashboard
        token: String,
        /// Automatically install as a system service
        #[arg(long)]
        install_service: bool,
    },
    /// Unregister this agent from Zexio Cloud
    Unregister,
    /// Install a software package (e.g., docker, redis, postgres)
    Install {
        /// Name of the package to install
        package: String,
        /// Optional: Exact shell command/script to execute
        #[arg(long)]
        command: Option<String>,
    },
    /// Uninstall a software package
    Uninstall {
        /// Name of the package to uninstall
        package: String,
        /// Optional: Exact shell command/script to execute
        #[arg(long)]
        command: Option<String>,
    },
    /// Manage Zexio Agent as a system service (daemon)
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },
}

#[derive(Subcommand)]
enum ServiceAction {
    /// Install as a system service
    Install,
    /// Uninstall system service
    Uninstall,
    /// Start background service
    Start,
    /// Stop background service
    Stop,
    /// Check service status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load configuration first (as requested)
    let settings = config::Settings::new()?;

    // 2. Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .init();

    // 3. Print startup banner & Debug Info
    print_banner();

    info!("🔧 Configuration loaded");
    info!("   Cloud API URL: {}", settings.cloud.api_url);
    info!(
        "   Management API: http://{}:{}",
        settings.server.host, settings.server.port
    );
    info!("   Mesh Proxy: port {}", settings.server.mesh_port);

    // 4. Parse CLI arguments
    let cli = Cli::parse();

    // Handle Commands
    match cli.command {
        Some(Commands::Login) => {
            info!("🔐 Authenticating with Zexio Cloud...");
            registration::interactive_login(&settings).await?;
            return Ok(());
        }
        Some(Commands::Connect {
            token,
            install_service,
        }) => {
            info!("🔗 Connecting to Zexio Cloud with token...");
            registration::connect_with_token(&settings, token).await?;

            if install_service {
                info!("⚙️  Automatically installing as system service...");
                daemon::handle_service(daemon::ServiceAction::Install).await?;
                daemon::handle_service(daemon::ServiceAction::Start).await?;
            }
            return Ok(());
        }
        Some(Commands::Unregister) => {
            info!("🔓 Unregistering from Zexio Cloud...");
            registration::unregister(&settings).await?;
            return Ok(());
        }
        Some(Commands::Service { action }) => {
            let daemon_action = match action {
                ServiceAction::Install => daemon::ServiceAction::Install,
                ServiceAction::Uninstall => daemon::ServiceAction::Uninstall,
                ServiceAction::Start => daemon::ServiceAction::Start,
                ServiceAction::Stop => daemon::ServiceAction::Stop,
                ServiceAction::Status => daemon::ServiceAction::Status,
            };
            daemon::handle_service(daemon_action).await?;
            return Ok(());
        }
        Some(Commands::Install { package, command }) => {
            if let Some(cmd) = command {
                info!(
                    "🛠️  Zexio is executing specialized command for {}: {}...",
                    package, cmd
                );
                match services::run_generic_command(&cmd).await {
                    Ok(stdout) => {
                        info!("✅ Successfully executed for {}:", package);
                        println!("{}", stdout);
                    }
                    Err(e) => error!("❌ Execution failed: {}", e),
                }
            } else {
                error!(
                    "❌ Error: No command provided. Use --command '...' to specify what to run."
                );
                info!("   Example: zexio install docker --command 'curl -fsSL https://get.docker.com | sudo sh'");
            }
            return Ok(());
        }
        Some(Commands::Uninstall { package, command }) => {
            if let Some(cmd) = command {
                info!(
                    "🗑️  Zexio is executing uninstall command for {}: {}...",
                    package, cmd
                );
                match services::run_generic_command(&cmd).await {
                    Ok(stdout) => {
                        info!("✅ Successfully executed uninstall for {}:", package);
                        println!("{}", stdout);
                    }
                    Err(e) => error!("❌ Uninstallation failed: {}", e),
                }
            } else {
                error!("❌ Error: No command provided for uninstallation. Use --command '...'");
            }
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
