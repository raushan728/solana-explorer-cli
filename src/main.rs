mod commands;
mod config;
mod rpc;

use clap::{Parser, Subcommand};
use colored::*;
use config::Config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cluster {
        #[command(subcommand)]
        action: ClusterAction,
    },
    Network,
    Account {
        address: String,
    },
    Tx {
        sig: String,
    },
}

#[derive(Subcommand)]
enum ClusterAction {
    Set { name: String },
    Get,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut cfg = Config::load();

    match cli.command {
        Commands::Cluster { action } => match action {
            ClusterAction::Set { name } => {
                let url = match name.as_str() {
                    "mainnet" | "m" => "https://api.mainnet-beta.solana.com",
                    "testnet" | "t" => "https://api.testnet.solana.com",
                    _ => "https://api.devnet.solana.com",
                };
                cfg.cluster = name;
                cfg.rpc_url = url.to_string();
                cfg.save()?;
                println!("✔ Cluster updated!");
            }
            ClusterAction::Get => {
                println!("Active Cluster: {}", cfg.cluster.cyan());
            }
        },
        Commands::Network => {
            let client = rpc::get_client(&cfg.rpc_url);
            commands::network::get_status(&client)?;
        }
        // _ => println!("Feature coming soon!"),
        Commands::Account { address } => {
            let client = rpc::get_client(&cfg.rpc_url);
            commands::account::get_info(&client, &address)?;
        }
        Commands::Tx { sig } => {
            let client = rpc::get_client(&cfg.rpc_url);
            commands::transaction::get_details(&client, &sig)?;
        }
    }
    Ok(())
}
