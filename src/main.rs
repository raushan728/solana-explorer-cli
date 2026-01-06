mod commands;
mod config;
mod rpc;

use clap::{Parser, Subcommand};
use colored::*;
use config::Config;

/// Raushan Explorer - CLI for Solana
#[derive(Parser)]
#[command(name = "raushan")]
#[command(about = "Ultimate Solana Terminal Explorer", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cluster Management: Set/Get active cluster, Genesis info, Health
    Cluster {
        #[command(subcommand)]
        action: ClusterAction,
    },
    /// Network Stats: TPS, Supply, Inflation, Epoch
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
    /// Account Inspection: Info, Tokens, History, Stake
    Account {
        #[command(subcommand)]
        action: AccountAction,
    },
    /// Transaction Inspection
    Tx {
        #[command(subcommand)]
        action: TxAction,
    },
    /// Block Inspection
    Block {
        #[command(subcommand)]
        action: BlockAction,
    },
    /// Validator Info
    Validator {
        #[command(subcommand)]
        action: ValidatorAction,
    },
    /// Token Mint/Supply Info
    Token {
        #[command(subcommand)]
        action: TokenAction,
    },
    /// Program Inspection
    Program {
        #[command(subcommand)]
        action: ProgramAction,
    },
}

#[derive(Subcommand)]
enum ClusterAction {
    Set { name: String },
    Get,
    Info,
    Health,
    Genesis,
}

#[derive(Subcommand)]
enum NetworkAction {
    Status,
    Supply,
    Inflation,
    Tps,
    EpochSchedule,
}

#[derive(Subcommand)]
enum AccountAction {
    Info { address: String },
    History { address: String },
    Tokens { address: String },
    Stake { address: String },
}

#[derive(Subcommand)]
enum TxAction {
    Info { sig: String },
    Logs { sig: String },
}

#[derive(Subcommand)]
enum BlockAction {
    Info { slot: u64 },
}

#[derive(Subcommand)]
enum ValidatorAction {
    List,
}

#[derive(Subcommand)]
enum TokenAction {
    MintInfo { address: String },
    List { owner: String }, // For convenience alias, though Account Tokens is preferred
}

#[derive(Subcommand)]
enum ProgramAction {
    Info { address: String },
    AccountsOwned { address: String },
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "ERROR:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut cfg = Config::load();

    match cli.command {
        Commands::Cluster { action } => match action {
            ClusterAction::Set { name } => commands::cluster::set(&mut cfg, name)?,
            ClusterAction::Get => commands::cluster::get(&cfg)?,
            ClusterAction::Info => {
                let client = rpc::get_client(&cfg.rpc_url);
                commands::cluster::info(&client)?;
            }
            ClusterAction::Health => {
                let client = rpc::get_client(&cfg.rpc_url);
                commands::cluster::health(&client)?;
            }
            ClusterAction::Genesis => {
                let client = rpc::get_client(&cfg.rpc_url);
                commands::cluster::genesis(&client)?;
            }
        },
        Commands::Network { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                NetworkAction::Status => commands::network::get_status(&client)?,
                NetworkAction::Supply => commands::network::get_supply(&client)?,
                NetworkAction::Inflation => commands::network::get_inflation(&client)?,
                NetworkAction::Tps => commands::network::get_tps(&client)?,
                NetworkAction::EpochSchedule => commands::network::get_epoch_schedule(&client)?,
            }
        }
        Commands::Account { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                AccountAction::Info { address } => commands::account::get_info(&client, &address)?,
                AccountAction::History { address } => {
                    commands::account::get_history(&client, &address)?
                }
                AccountAction::Tokens { address } => {
                    commands::account::get_tokens(&client, &address)?
                }
                AccountAction::Stake { address } => {
                    commands::account::get_stake(&client, &address)?
                }
            }
        }
        Commands::Tx { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                TxAction::Info { sig } => commands::transaction::get_details(&client, &sig)?,
                TxAction::Logs { sig } => commands::transaction::get_logs(&client, &sig)?,
            }
        }
        Commands::Block { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                BlockAction::Info { slot } => commands::block::get_block(&client, slot)?,
            }
        }
        Commands::Validator { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                ValidatorAction::List => commands::validator::get_validators(&client)?,
            }
        }
        Commands::Token { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                TokenAction::MintInfo { address } => {
                    commands::token::get_mint_info(&client, &address)?
                }
                TokenAction::List { owner } => {
                    commands::token::get_token_accounts(&client, &owner)?
                }
            }
        }
        Commands::Program { action } => {
            let client = rpc::get_client(&cfg.rpc_url);
            match action {
                ProgramAction::Info { address } => commands::program::get_info(&client, &address)?,
                ProgramAction::AccountsOwned { address } => {
                    commands::program::get_accounts(&client, &address)?
                }
            }
        }
    }
    Ok(())
}
