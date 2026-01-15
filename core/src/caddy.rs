use crate::config::CaddySettings;
use anyhow::{Context, Result};
use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tracing::{info, warn};

pub struct Caddy {
    settings: CaddySettings,
}

impl Caddy {
    pub fn new(settings: CaddySettings) -> Self {
        Self { settings }
    }

    pub fn add_domain(&self, domain: &str, _project_id: &str, port: u16) -> Result<()> {
        let caddyfile_path = &self.settings.caddyfile_path;
        
        // Ensure file exists
        if !std::path::Path::new(caddyfile_path).exists() {
            fs::write(caddyfile_path, "")?;
        }

        let config_block = format!(
            "\n{} {{\n    reverse_proxy localhost:{}\n}}\n",
            domain, port
        );

        let current_content = fs::read_to_string(caddyfile_path)?;
        if current_content.contains(&format!("{} {{", domain)) {
            info!("Domain {} already exists in Caddyfile, skipping", domain);
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(caddyfile_path)
            .context("Failed to open Caddyfile for appending")?;

        file.write_all(config_block.as_bytes())
            .context("Failed to write to Caddyfile")?;

        Ok(())
    }

    pub fn remove_domain(&self, domain: &str) -> Result<()> {
        let caddyfile_path = &self.settings.caddyfile_path;
        if !std::path::Path::new(caddyfile_path).exists() {
            return Ok(());
        }

        let current_content = fs::read_to_string(caddyfile_path)?;
        
        // Naive parser: find domain block and remove it
        // Format assumption: "domain.com {" ... "}"
        // This is fragile but works for the predictable format we generate.
        let start_marker = format!("{} {{", domain);
        if let Some(start_idx) = current_content.find(&start_marker) {
            // Find the closing brace for this block
            // Since our block is simple: "\n    reverse_proxy localhost:PORT\n}\n"
            // We search for the next "}\n" after start_idx
            if let Some(end_idx_offset) = current_content[start_idx..].find("\n}\n") {
                 let end_idx = start_idx + end_idx_offset + 3; // +3 for "\n}\n" length
                 
                 let new_content = format!("{}{}", &current_content[..start_idx], &current_content[end_idx..]);
                 fs::write(caddyfile_path, new_content.trim())?;
                 info!("Removed domain {} from Caddyfile", domain);
            } else {
                 warn!("Could not find closing brace for domain {}, skipping removal", domain);
            }
        } else {
             info!("Domain {} not found in Caddyfile", domain);
        }

        Ok(())
    }

    pub fn reload(&self) -> Result<()> {
        info!("Reloading Caddy...");
        let status = Command::new("caddy")
            .arg("reload")
            .arg("--config")
            .arg(&self.settings.caddyfile_path) // This might be wrong if this is an included file.
            // If `caddyfile_path` is the MAIN Caddyfile, this is correct.
            // If it is an imported file, we should reload the MAIN Caddyfile or just `caddy reload` if running as service.
            // Assuming `caddy reload` picks up the active config or adapter.
            // A safer bet for systemd managed caddy is `systemctl reload caddy`, but we might not have sudo.
            // Let's assume we run `caddy run` or `caddy start` manually or user has configured it.
            // The prompt says "Run `caddy reload`".
            .status()
            .context("Failed to execute caddy reload")?;

        if !status.success() {
            anyhow::bail!("Caddy reload failed");
        }
        
        Ok(())
    }
}
