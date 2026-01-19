use crate::config::Settings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{error, info};

#[derive(Serialize)]
struct ConnectDto {
    token: String,
    name: Option<String>,
    os_info: OsInfo,
}

#[derive(Serialize)]
struct OsInfo {
    hostname: String,
    os_type: String,
    os_arch: String,
}

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

#[derive(Serialize, Deserialize, Clone)]
pub struct Identity {
    pub worker_id: String,
    pub secret_key: String,
    #[serde(default = "default_relay")]
    pub relay_url: String,
}

fn default_relay() -> String {
    "wss://relay.zexio.io:443".to_string()
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
        let api_url = format!("{}/api/nodes/heartbeat", settings.cloud.api_url);

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
    let api_url = format!("{}/api/nodes/register", settings.cloud.api_url);

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
        relay_url: "wss://relay.zexio.io:443".to_string(), // Default during initial handshake if not provided
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
    let api_url = format!("{}/api/nodes/unregister", settings.cloud.api_url);

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

pub async fn interactive_login(settings: &Settings) -> anyhow::Result<()> {
    let identity_path = &settings.secrets.identity_path;

    // Check if already logged in
    if Path::new(identity_path).exists() {
        info!("⚠️  You are already authenticated.");
        info!("   Identity: {}", identity_path);

        // Ask if user wants to re-authenticate
        println!("\n🔄 Do you want to unregister the current node and log in again? (y/N): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            info!("✅ Keeping existing identity.");
            return Ok(());
        }

        info!("🗑️  Unregistering current node...");
        if let Err(e) = unregister(settings).await {
            error!(
                "⚠️  Failed to unregister old node from cloud: {}. Proceeding anyway...",
                e
            );
        }
        fs::remove_file(identity_path)?;
        info!("🗑️  Old identity removed.");
    }

    // Prompt for provisioning token
    println!("\n📋 Enter your provisioning token (from Zexio Dashboard):");
    println!("   Format: zxp_...");
    print!("   Token: ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut token = String::new();
    std::io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();

    if token.is_empty() {
        return Err(anyhow::anyhow!("Token cannot be empty"));
    }

    if token.len() < 8 {
        return Err(anyhow::anyhow!(
            "Invalid token format. Expected at least 8 characters."
        ));
    }

    info!("🔐 Authenticating with Zexio Cloud...");

    // Gather System Info
    let hostname = hostname::get()?.to_string_lossy().to_string();
    let os = sysinfo::System::name().unwrap_or("unknown".to_string());
    let arch = sysinfo::System::cpu_arch().unwrap_or("unknown".to_string());

    let client = reqwest::Client::new();
    let dto = ConnectDto {
        token: token.clone(),
        name: None,
        os_info: OsInfo {
            hostname,
            os_type: os,
            os_arch: arch,
        },
    };

    // Send Registration Request
    let api_url = format!("{}/api/nodes/connect", settings.cloud.api_url);

    let res = client.post(&api_url).json(&dto).send().await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        error!("❌ Authentication failed: {}", err_text);
        return Err(anyhow::anyhow!("Authentication failed: {}", err_text));
    }

    let res_text = res.text().await?;
    let response: serde_json::Value = serde_json::from_str(&res_text)?;
    let data = response
        .get("data")
        .ok_or_else(|| anyhow::anyhow!("Invalid response: missing data"))?;

    // Save Identity
    let identity = Identity {
        worker_id: data["node_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing node_id"))?
            .to_string(),
        secret_key: data["node_secret"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing node_secret"))?
            .to_string(),
        relay_url: data["relay_url"]
            .as_str()
            .unwrap_or("wss://relay.zexio.io:443")
            .to_string(),
    };

    let identity_json = serde_json::to_string_pretty(&identity)?;
    fs::write(identity_path, identity_json)?;

    // Secure the file (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(identity_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(identity_path, perms)?;
    }

    info!("✅ Authentication successful!");
    info!("   Worker ID: {}", identity.worker_id);
    info!("   Identity saved: {}", identity_path);
    info!("");
    info!("💡 You can now run: zexio up <port>");

    Ok(())
}

pub async fn connect_with_token(settings: &Settings, token: String) -> anyhow::Result<()> {
    let identity_path = &settings.secrets.identity_path;

    if Path::new(identity_path).exists() {
        info!("⚠️  You are already authenticated.");
        info!("   Current identity: {}", identity_path);
        println!("\n🔄 Do you want to unregister the current node and connect with the new token? (y/N): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            info!("✅ Keeping existing identity.");
            return Ok(());
        }

        info!("🗑️  Unregistering current node...");
        if let Err(e) = unregister(settings).await {
            error!(
                "⚠️  Failed to unregister old node from cloud: {}. Proceeding anyway...",
                e
            );
        }
        let _ = fs::remove_file(identity_path);
    }

    info!("🔗 Connecting to Zexio Cloud...");

    // Gather System Info
    let hostname = hostname::get()?.to_string_lossy().to_string();
    let os_type = sysinfo::System::name().unwrap_or("unknown".to_string());
    let os_arch = sysinfo::System::cpu_arch().unwrap_or("unknown".to_string());

    let dto = ConnectDto {
        token,
        name: None, // Will use server-side default or pending name
        os_info: OsInfo {
            hostname,
            os_type,
            os_arch,
        },
    };

    let client = reqwest::Client::new();
    let api_url = format!("{}/api/nodes/connect", settings.cloud.api_url);

    let res = client.post(&api_url).json(&dto).send().await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        error!("❌ Connection failed: {}", err_text);
        return Err(anyhow::anyhow!("Connection failed: {}", err_text));
    }

    let res_text = res.text().await?;
    let response: serde_json::Value = serde_json::from_str(&res_text)?;

    // The response schema from NestJS is usually { success: true, data: { ... } }
    let data = response
        .get("data")
        .ok_or_else(|| anyhow::anyhow!("Invalid response: missing data"))?;

    let identity = Identity {
        worker_id: data["node_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing node_id"))?
            .to_string(),
        secret_key: data["node_secret"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing node_secret"))?
            .to_string(),
        relay_url: data["relay_url"]
            .as_str()
            .unwrap_or("wss://relay.zexio.io:443")
            .to_string(),
    };

    let identity_json = serde_json::to_string_pretty(&identity)?;
    fs::write(identity_path, identity_json)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(identity_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(identity_path, perms)?;
    }

    info!("✅ Connected successfully!");
    info!("   Node ID: {}", identity.worker_id);
    info!("   Identity saved: {}", identity_path);
    info!("");
    info!("🚀 Start the agent with: zexio up <port>");

    Ok(())
}
