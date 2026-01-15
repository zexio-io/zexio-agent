use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use anyhow::{Context, Result};
use rand::Rng;
use std::path::Path;

#[derive(Clone)]
pub struct Crypto {
    key: Vec<u8>,
}

impl Crypto {
    pub fn new(master_key_path: &str) -> Result<Self> {
        // Auto-generate master key if not exists
        let key_hex = if Path::new(master_key_path).exists() {
            std::fs::read_to_string(master_key_path)
                .with_context(|| format!("Failed to read master key from {}", master_key_path))?
                .trim()
                .to_string()
        } else {
            // Generate new master key
            let key = Self::generate_master_key();
            
            // Create parent directory if not exists
            if let Some(parent) = Path::new(master_key_path).parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory for master key: {:?}", parent))?;
            }
            
            // Write key to file
            std::fs::write(master_key_path, &key)
                .with_context(|| format!("Failed to write master key to {}", master_key_path))?;
            
            tracing::info!("Generated new master key at {}", master_key_path);
            key
        };

        let key_bytes = hex::decode(&key_hex)
            .with_context(|| "Master key must be 64 hex characters (32 bytes)")?;

        if key_bytes.len() != 32 {
            anyhow::bail!("Master key must be exactly 32 bytes");
        }

        Ok(Self { key: key_bytes })
    }

    /// Generate a new random 32-byte master key as hex string
    fn generate_master_key() -> String {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        hex::encode(key)
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            anyhow::bail!("Invalid encrypted data: too short");
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
    }

    // HMAC-SHA256 verification
    pub fn verify_signature(secret: &str, body: &[u8], signature_hex: &str) -> bool {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = match <HmacSha256 as Mac>::new_from_slice(secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        
        mac.update(body);
        
        let expected_bytes = mac.finalize().into_bytes();
        let expected_hex = hex::encode(expected_bytes);

        // Remove "sha256=" prefix if present
        let clean_sig = signature_hex.trim_start_matches("sha256=");
        
        expected_hex == clean_sig
    }
}
