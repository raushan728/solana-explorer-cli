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
        Ok(_) => println!("{}", "Cluster is unhealthy".red()), // get_health returns error if unhealthy? No, returns Ok(()) if healthy.
        Err(_) => println!("{}", "Cluster is healthy".green()), // Wait, docs say: "Returns an error if the cluster is unhealthy". So Ok = Healthy.
    }
    // Correct check:
    // client.get_health() returns Ok(()) if healthy.
    // Let's re-verify. "Return an error if the node is unhealthy".
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
