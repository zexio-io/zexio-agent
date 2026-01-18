use std::fs;
use std::process::Command;
use tracing::{info, error};
use anyhow::Result;

pub enum ServiceAction {
    Install,
    Uninstall,
    Start,
    Stop,
    Status,
}

pub async fn handle_service(action: ServiceAction) -> Result<()> {
    match action {
        ServiceAction::Install => install_service().await,
        ServiceAction::Uninstall => uninstall_service().await,
        ServiceAction::Start => start_service().await,
        ServiceAction::Stop => stop_service().await,
        ServiceAction::Status => status_service().await,
    }
}

async fn install_service() -> Result<()> {
    let exe_path = std::env::current_exe()?;
    let exe_str = exe_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid executable path"))?;

    #[cfg(target_os = "linux")]
    {
        info!("Installing systemd service for Linux...");
        let service_content = format!(
            r#"[Unit]
Description=Zexio Agent
After=network.target

[Service]
ExecStart={}
Restart=always
User=root

[Install]
WantedBy=multi-user.target
"#,
            exe_str
        );

        let service_path = "/etc/systemd/system/zexio.service";
        fs::write(service_path, service_content)?;
        
        info!("Reloading systemd daemon...");
        Command::new("systemctl").arg("daemon-reload").status()?;
        info!("Enabling zexio service...");
        Command::new("systemctl").arg("enable").arg("zexio").status()?;
        
        info!("✅ Zexio Agent service installed successfully.");
        info!("   Start it with: zexio service start");
    }

    #[cfg(target_os = "macos")]
    {
        info!("Installing launchd service for macOS (User Level)...");
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let plist_dir = home.join("Library/LaunchAgents");
        fs::create_dir_all(&plist_dir)?;
        
        let plist_path = plist_dir.join("io.zexio.agent.plist");
        let log_dir = home.join("Library/Logs/Zexio");
        fs::create_dir_all(&log_dir)?;

        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.zexio.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{}</string>
    <key>StandardErrorPath</key>
    <string>{}</string>
</dict>
</plist>
"#,
            exe_str,
            log_dir.join("agent.log").to_string_lossy(),
            log_dir.join("agent.err.log").to_string_lossy()
        );

        fs::write(&plist_path, plist_content)?;
        
        info!("✅ Zexio Agent service installed to: {}", plist_path.display());
        info!("   Start it with: zexio service start");
    }

    #[cfg(target_os = "windows")]
    {
        info!("Installing Windows Service...");
        let status = Command::new("sc.exe")
            .arg("create")
            .arg("zexio")
            .arg(format!("binPath= {}", exe_str))
            .arg("start= auto")
            .arg("DisplayName= Zexio Agent")
            .status()?;

        if status.success() {
            info!("✅ Zexio Agent service installed successfully.");
        } else {
            return Err(anyhow::anyhow!("Failed to install Windows service"));
        }
    }

    Ok(())
}

async fn uninstall_service() -> Result<()> {
    info!("Uninstalling Zexio Agent service...");
    
    // Always try to stop first to avoid errors
    let _ = stop_service().await;

    #[cfg(target_os = "linux")]
    {
        info!("Disabling and removing systemd service...");
        Command::new("systemctl").arg("disable").arg("zexio").status()?;
        let service_path = "/etc/systemd/system/zexio.service";
        if std::path::Path::new(service_path).exists() {
            fs::remove_file(service_path)?;
        }
        Command::new("systemctl").arg("daemon-reload").status()?;
    }

    #[cfg(target_os = "macos")]
    {
        info!("Removing launchd service...");
        if let Some(home) = dirs::home_dir() {
            let plist_path = home.join("Library/LaunchAgents/io.zexio.agent.plist");
            if plist_path.exists() {
                fs::remove_file(plist_path)?;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        info!("Deleting Windows Service...");
        Command::new("sc.exe").arg("delete").arg("zexio").status()?;
    }

    info!("✅ Zexio Agent service uninstalled successfully.");
    Ok(())
}

async fn start_service() -> Result<()> {
    info!("Starting Zexio Agent service...");
    
    #[cfg(target_os = "linux")]
    {
        Command::new("systemctl").arg("start").arg("zexio").status()?;
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            let plist_path = home.join("Library/LaunchAgents/io.zexio.agent.plist");
            Command::new("launchctl").arg("load").arg(plist_path).status()?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("sc.exe").arg("start").arg("zexio").status()?;
    }

    info!("✅ Service start command sent.");
    Ok(())
}

async fn stop_service() -> Result<()> {
    info!("Stopping Zexio Agent service...");
    
    #[cfg(target_os = "linux")]
    {
        Command::new("systemctl").arg("stop").arg("zexio").status()?;
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            let plist_path = home.join("Library/LaunchAgents/io.zexio.agent.plist");
            Command::new("launchctl").arg("unload").arg(plist_path).status()?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("sc.exe").arg("stop").arg("zexio").status()?;
    }

    info!("✅ Service stop command sent.");
    Ok(())
}

async fn status_service() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("systemctl").arg("status").arg("zexio").status()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("launchctl").arg("list").arg("io.zexio.agent").status()?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("sc.exe").arg("query").arg("zexio").status()?;
    }

    Ok(())
}
