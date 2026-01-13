use crate::config::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{info, error};

#[derive(Serialize)]
struct RegisterDto {
    token: String,
    hostname: String,
    arch: String,
    os: String,
}

#[derive(Deserialize)]
struct RegisterResponse {
    worker_id: String,
    secret_key: String,
}

#[derive(Serialize, Deserialize)]
struct Identity {
    worker_id: String,
    secret_key: String,
}

pub async fn handshake(settings: &Settings) -> anyhow::Result<()> {
    let identity_path = "/etc/zexio/identity.json";

    // 1. Check if already registered
    if Path::new(identity_path).exists() {
        info!("Identity found. Agent is already registered.");
        return Ok(());
    }

    // 2. Check for Provisioning Token
    // 2. Check for Provisioning Token
    // Priority: Env/Config (settings.cloud.token) -> File (/etc/zexio/provisioning_token)
    let token_path = "/etc/zexio/provisioning_token";
    
    let token = if let Some(t) = &settings.cloud.token {
        info!("Using provisioning token from configuration/env.");
        t.clone()
    } else if std::path::Path::new(token_path).exists() {
        info!("Found provisioning token file.");
        fs::read_to_string(token_path)?.trim().to_string()
    } else {
        info!("No provisioning token found (Env or File). Waiting for manual configuration.");
        return Ok(());
    };
    info!("Found provisioning token. Attempting registration...");

    // 3. Gather System Info
    let hostname = hostname::get()?.to_string_lossy().to_string();
    let _info = sysinfo::System::new_all();
    let os = sysinfo::System::name().unwrap_or("unknown".to_string());
    let arch = sysinfo::System::cpu_arch().unwrap_or("unknown".to_string());

    let client = reqwest::Client::new();
    let dto = RegisterDto {
        token,
        hostname,
        os,
        arch,
    };

    // 4. Send Registration Request
    let api_url = format!("{}/workers/register", settings.cloud.api_url);
    
    let res = client.post(api_url)
        .header("X-Zexio-Token", &dto.token)
        .json(&dto)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        error!("Registration failed: {}", err_text);
        return Err(anyhow::anyhow!("Registration failed: {}", err_text));
    }

    let response: RegisterResponse = res.json().await?;

    // 5. Save Identity
    let identity = Identity {
        worker_id: response.worker_id,
        secret_key: response.secret_key,
    };

    let identity_json = serde_json::to_string_pretty(&identity)?;
    fs::write(identity_path, identity_json)?;
    
    // Secure the file
    // Secure the file (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(identity_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(identity_path, perms)?;
    }

    info!("Registration successful! Identity saved.");

    // 6. Cleanup Token (Security Best Practice)
    let _ = fs::remove_file(token_path);

    Ok(())
}
