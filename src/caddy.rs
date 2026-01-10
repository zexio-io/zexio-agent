use crate::config::CaddySettings;
use anyhow::{Context, Result};
use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::Write;
use tracing::info;

pub struct Caddy {
    settings: CaddySettings,
}

impl Caddy {
    pub fn new(settings: CaddySettings) -> Self {
        Self { settings }
    }

    pub fn add_domain(&self, domain: &str, project_id: &str, port: u16) -> Result<()> {
        let caddyfile_path = &self.settings.caddyfile_path;
        
        // Ensure file exists
        if !std::path::Path::new(caddyfile_path).exists() {
            fs::write(caddyfile_path, "")?;
        }

        let config_block = format!(
            "\n{} {{\n    reverse_proxy localhost:{}\n}}\n",
            domain, port
        );

        // Simple check to avoid duplicate entries (naive, but works for MVP)
        // Ideally we should parse Caddyfile or use JSON API
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
