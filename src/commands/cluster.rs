use crate::config::Config;
use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;

pub fn set(cfg: &mut Config, name: String) -> Result<()> {
    let url = match name.as_str() {
        "mainnet" | "mainnet-beta" | "m" => "https://api.mainnet-beta.solana.com",
        "testnet" | "t" => "https://api.testnet.solana.com",
        _ => "https://api.devnet.solana.com",
    };
    cfg.cluster = name;
    cfg.rpc_url = url.to_string();
    cfg.save()?;
    println!("{} Cluster updated to: {}", "✔".green(), cfg.cluster.cyan());
    Ok(())
}

pub fn get(cfg: &Config) -> Result<()> {
    println!(
        "Active Cluster: {} ({})",
        cfg.cluster.cyan().bold(),
        cfg.rpc_url
    );
    Ok(())
}

pub fn info(client: &RpcClient) -> Result<()> {
    let version = client.get_version()?;
    println!("\n{}", "--- Cluster Info ---".bold().cyan());
    println!("{:<20} : {}", "Solana Core", version.solana_core.green());
    println!(
        "{:<20} : {}",
        "Feature Set",
        version.feature_set.unwrap_or_default()
    );
    Ok(())
}

pub fn health(client: &RpcClient) -> Result<()> {
    match client.get_health() {
        Ok(_) => println!("{}", "Cluster Status: HEALTHY".green().bold()),
        Err(e) => println!("{} ({})", "Cluster Status: UNHEALTHY".red().bold(), e),
    }
    Ok(())
}

pub fn genesis(client: &RpcClient) -> Result<()> {
    let hash = client.get_genesis_hash()?;
    println!("Genesis Hash: {}", hash.to_string().yellow());
    Ok(())
}

pub fn get_nodes(client: &RpcClient) -> Result<()> {
    let nodes = client.get_cluster_nodes()?;
    println!(
        "\n{}",
        format!("--- Cluster Nodes ({}) ---", nodes.len())
            .bold()
            .cyan()
    );
    println!("{:<45} | {:<20} | {}", "Pubkey", "Version", "RPC");
    println!("{}", "-".repeat(80));
    for node in nodes.iter().take(20) {
        let rpc = node
            .rpc
            .map(|s| s.to_string())
            .unwrap_or_else(|| "-".to_string());
        let ver = node
            .version
            .clone()
            .unwrap_or_else(|| "Unknown".to_string());
        println!("{:<45} | {:<20} | {}", node.pubkey, ver, rpc);
    }
    if nodes.len() > 20 {
        println!("... and {} more.", nodes.len() - 20);
    }
    Ok(())
}
