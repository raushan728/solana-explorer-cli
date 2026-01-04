use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cluster: String,
    pub rpc_url: String,
}

impl Config {
    // Default config: Devnet
    pub fn default() -> Self {
        Self {
            cluster: "devnet".to_string(),
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }

    pub fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".raushan_config.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_path();
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}
