use crate::config::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{error, info};

#[derive(Serialize)]
struct RegisterDto {
    token: String,
    worker_id: Option<String>,
    hostname: String,
    arch: String,
    os: String,
}

#[derive(Deserialize)]
struct RegisterData {
    worker_id: String,
    secret_key: String,
}

#[derive(Deserialize)]
struct RegisterResponse {
    data: RegisterData,
}

#[derive(Serialize, Deserialize)]
struct Identity {
    worker_id: String,
    secret_key: String,
}
pub async fn handshake(settings: &Settings) -> anyhow::Result<()> {
    let identity_path = &settings.secrets.identity_path;
    info!("Checking for existing identity at: {}", identity_path);

    // 1. Check if already registered
    if Path::new(identity_path).exists() {
        info!("Identity found. Verifying with cloud...");
        let identity_json = fs::read_to_string(identity_path)?;
        let identity: Identity = serde_json::from_str(&identity_json)?;

        let client = reqwest::Client::new();
        let api_url = format!("{}/workers/heartbeat", settings.cloud.api_url);

        let res = client
            .post(api_url)
            .json(&serde_json::json!({
                "worker_id": identity.worker_id,
                "secret": identity.secret_key,
            }))
            .send()
            .await?;

        if res.status().is_success() {
            info!("Identity verified. Agent is online.");
            return Ok(()).map_err(|e: anyhow::Error| e);
        } else if res.status() == reqwest::StatusCode::FORBIDDEN
            || res.status() == reqwest::StatusCode::NOT_FOUND
        {
            error!("Identity no longer valid in cloud. Resetting local identity.");
            let _ = fs::remove_file(identity_path);
        } else {
            let res_text = res.text().await?;
            if settings.debug {
                info!("DEBUG: Heartbeat response: {}", res_text);
            }
            error!(
                "Heartbeat failed: {}. Continuing with existing identity.",
                res_text
            );
            return Ok(()); // Don't block startup if cloud is just down
        }
    }

    // 2. Check for Provisioning Token
    // Priority: Env/Config (settings.cloud.token) -> File (settings.secrets.provisioning_token_path)
    let token_path = &settings.secrets.provisioning_token_path;

    let token = if let Some(t) = &settings.cloud.token {
        info!("Using provisioning token from configuration/env.");
        t.clone()
    } else if std::path::Path::new(token_path).exists() {
        info!("Found provisioning token file.");
        fs::read_to_string(token_path)?.trim().to_string()
    } else {
        info!("No provisioning token found (Env or File).");
        info!("💡 TIP: Use an active token from the Zexio Dashboard to connect to the cloud.");
        return Ok(());
    };
    info!("Found provisioning token. Attempting registration...");

    // 3. Gather System Info
    let hostname = hostname::get()?.to_string_lossy().to_string();
    let _info = sysinfo::System::new_all();
    let os = sysinfo::System::name().unwrap_or("unknown".to_string());
    let arch = sysinfo::System::cpu_arch().unwrap_or("unknown".to_string());

    info!(
        "System Info: Hostname={}, OS={}, Arch={}",
        hostname, os, arch
    );

    let client = reqwest::Client::new();
    let dto = RegisterDto {
        token,
        worker_id: settings.cloud.worker_id.clone(),
        hostname,
        os,
        arch,
    };

    // 4. Send Registration Request
    let api_url = format!("{}/workers/register", settings.cloud.api_url);

    let res = client
        .post(api_url)
        .header("X-Zexio-Token", &dto.token)
        .json(&dto)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        error!("Registration failed: {}", err_text);
        info!("💡 TIP: Your token might be expired or invalid. Please generate a new one from the Zexio Dashboard.");
        return Err(anyhow::anyhow!("Registration failed: {}", err_text));
    }

    let res_text = res.text().await?;
    if settings.debug {
        info!("DEBUG: Registration response: {}", res_text);
    }
    let response: RegisterResponse = serde_json::from_str(&res_text)?;

    // 5. Save Identity
    let identity = Identity {
        worker_id: response.data.worker_id,
        secret_key: response.data.secret_key,
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

    info!(
        "Registration successful! Assigned Worker ID: {}",
        identity.worker_id
    );
    info!("Identity saved at {}.", identity_path);

    // 6. Cleanup Token (Security Best Practice)
    let _ = fs::remove_file(token_path);

    Ok(())
}

pub async fn unregister(settings: &Settings) -> anyhow::Result<()> {
    let identity_path = &settings.secrets.identity_path;

    if !Path::new(identity_path).exists() {
        return Err(anyhow::anyhow!(
            "No identity found. Agent is not registered."
        ));
    }

    let identity_json = fs::read_to_string(identity_path)?;
    let identity: Identity = serde_json::from_str(&identity_json)?;

    info!("Unregistering agent {} from cloud...", identity.worker_id);

    let client = reqwest::Client::new();
    let api_url = format!("{}/workers/unregister", settings.cloud.api_url);

    let res = client
        .post(api_url)
        .json(&serde_json::json!({
            "worker_id": identity.worker_id,
            "secret": identity.secret_key,
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        error!("Unregistration failed on server: {}", err_text);
        return Err(anyhow::anyhow!("Unregistration failed: {}", err_text));
    }

    if settings.debug {
        let res_text = res.text().await?;
        info!("DEBUG: Unregistration response: {}", res_text);
    }

    // Delete local identity
    fs::remove_file(identity_path)?;
    info!("Agent unregistered successfully. Local identity deleted.");

    Ok(())
}
