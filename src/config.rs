use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure.
///
/// Stores:
/// - `cluster`: The name of the active cluster (e.g., "devnet", "mainnet").
/// - `rpc_url`: The full HTTP/HTTPS URL for the RPC endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cluster: String,
    pub rpc_url: String,
}

impl Config {
    /// Create a default configuration (connected to Devnet).
    pub fn default() -> Self {
        Self {
            cluster: "devnet".to_string(),
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }

    /// Determine the configuration file path.
    /// Default: `~/.raushan_config.json`
    pub fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".raushan_config.json");
        path
    }

    /// Load configuration from disk.
    /// Returns default config configuration if file is missing or invalid.
    pub fn load() -> Self {
        let path = Self::get_path();
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }
        Self::default()
    }

    /// Save current configuration to disk.
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}
