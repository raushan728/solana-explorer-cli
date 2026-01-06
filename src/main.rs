mod commands;
mod config;
mod rpc;

use clap::{Parser, Subcommand};
use colored::*;
use config::Config;

/// Raushan Explorer - A production-grade Solana CLI Toolkit
#[derive(Parser)]
#[command(name = "raushan")]
#[command(about = "Ultimate Solana Terminal Explorer", long_about = None)]
#[command(version)]
struct Cli {
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // --- CLUSTER ---
    /// Set the active cluster (devnet, testnet, mainnet-beta).
    #[command(name = "cluster-set")]
    ClusterSet { name: String },

    /// Get the currently active cluster configuration.
    #[command(name = "cluster-get")]
    ClusterGet,

    /// Show detailed information about the current cluster version and features.
    #[command(name = "cluster-info")]
    ClusterInfo,

    /// Check the health status of the current cluster.
    #[command(name = "cluster-health")]
    ClusterHealth,

    /// Display the Genesis Hash of the current cluster.
    #[command(name = "cluster-genesis")]
    ClusterGenesis,

    /// List known nodes in the cluster (Gossip).
    #[command(name = "cluster-nodes")]
    ClusterNodes,

    // --- NETWORK ---
    /// Detailed dashboard of the current network status (Epoch, Slot, Height).
    #[command(name = "network-status")]
    NetworkStatus,

    /// Real-time Transactions Per Second (TPS) tracker with performance overview.
    #[command(name = "network-tps")]
    NetworkTps,

    /// Show total, circulating, and non-circulating SOL supply.
    #[command(name = "network-supply")]
    NetworkSupply,

    /// Display current inflation rates (Total, Validator, Foundation).
    #[command(name = "network-inflation")]
    NetworkInflation,

    /// Show Epoch schedule and leader schedule offsets.
    #[command(name = "network-epoch-info")]
    NetworkEpochInfo,

    // --- ACCOUNT ---
    /// Fetch and display detailed account information and balance.
    #[command(name = "account-info")]
    AccountInfo { address: String },

    /// Show SPL Token accounts owned by this address.
    #[command(name = "account-tokens")]
    AccountTokens { address: String },

    /// Analyze stake accounts owned/managed by this address.
    #[command(name = "account-stake")]
    AccountStake { address: String },

    /// List recent transaction history for the account.
    #[command(name = "account-history")]
    AccountHistory { address: String },

    // --- TRANSACTION ---
    /// Breakdown of a transaction (Status, Fee, Logs, Instructions).
    #[command(name = "tx-info")]
    TxInfo { sig: String },

    /// Extract and display only the logs of a transaction.
    #[command(name = "tx-logs")]
    TxLogs { sig: String },

    /// Simulate a transaction (Dry run) - *Not fully implemented yet*.
    #[command(name = "tx-simulate")]
    TxSimulate { sig: String },

    /// Check prioritization fees for a transaction or block.
    #[command(name = "tx-priority")]
    TxPriority,

    // --- BLOCK ---
    /// Deep dive into a specific block by slot number.
    #[command(name = "block-info")]
    BlockInfo { slot: u64 },

    /// List transactions within a specific block.
    #[command(name = "block-transactions")]
    BlockTransactions { slot: u64 },

    /// Show rewards distribution for a block.
    #[command(name = "block-rewards")]
    BlockRewards { slot: u64 },

    // --- VALIDATOR ---
    /// List top active validators by stake.
    #[command(name = "validator-list")]
    ValidatorList,

    /// Get detailed info about a specific validator identity.
    #[command(name = "validator-info")]
    ValidatorInfo { identity: String },

    // --- TOKEN ---
    /// Get details of a Token Mint (Supply, Decimals, Authorities).
    #[command(name = "token-mint")]
    TokenMint { address: String },

    /// List largest holders of a specific Token Mint.
    #[command(name = "token-holders")]
    TokenHolders { address: String },

    // --- STAKE ---
    /// Inspect a specific stake account state and activation.
    #[command(name = "stake-account-info")]
    StakeAccountInfo { address: String },
    // --- PROGRAM ---
    /// details about a specific program (Owner, Data Size, Executable).
    #[command(name = "program-info")]
    ProgramInfo { address: String },

    /// List accounts owned by a specific program.
    #[command(name = "program-accounts")]
    ProgramAccounts { address: String },
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

    if let Some(cmd) = cli.command {
        match cmd {
            // Cluster
            Commands::ClusterSet { name } => commands::cluster::set(&mut cfg, name)?,
            Commands::ClusterGet => commands::cluster::get(&cfg)?,
            Commands::ClusterInfo => commands::cluster::info(&rpc::get_client(&cfg.rpc_url))?,
            Commands::ClusterHealth => commands::cluster::health(&rpc::get_client(&cfg.rpc_url))?,
            Commands::ClusterGenesis => commands::cluster::genesis(&rpc::get_client(&cfg.rpc_url))?,
            Commands::ClusterNodes => commands::cluster::get_nodes(&rpc::get_client(&cfg.rpc_url))?,

            // Network
            Commands::NetworkStatus => {
                commands::network::get_status(&rpc::get_client(&cfg.rpc_url))?
            }
            Commands::NetworkTps => commands::network::get_tps(&rpc::get_client(&cfg.rpc_url))?,
            Commands::NetworkSupply => {
                commands::network::get_supply(&rpc::get_client(&cfg.rpc_url))?
            }
            Commands::NetworkInflation => {
                commands::network::get_inflation(&rpc::get_client(&cfg.rpc_url))?
            }
            Commands::NetworkEpochInfo => {
                commands::network::get_epoch_schedule(&rpc::get_client(&cfg.rpc_url))?
            }

            // Account
            Commands::AccountInfo { address } => {
                commands::account::get_info(&rpc::get_client(&cfg.rpc_url), &address)?
            }
            Commands::AccountTokens { address } => {
                commands::account::get_tokens(&rpc::get_client(&cfg.rpc_url), &address)?
            }
            Commands::AccountStake { address } => {
                commands::account::get_stake(&rpc::get_client(&cfg.rpc_url), &address)?
            }
            Commands::AccountHistory { address } => {
                commands::account::get_history(&rpc::get_client(&cfg.rpc_url), &address)?
            }

            // Transaction
            Commands::TxInfo { sig } => {
                commands::transaction::get_details(&rpc::get_client(&cfg.rpc_url), &sig)?
            }
            Commands::TxLogs { sig } => {
                commands::transaction::get_logs(&rpc::get_client(&cfg.rpc_url), &sig)?
            }
            Commands::TxSimulate { sig: _ } => println!("Simulation feature coming soon."),
            Commands::TxPriority => println!("Priority fees feature coming soon."),

            // Block
            Commands::BlockInfo { slot } => {
                commands::block::get_block(&rpc::get_client(&cfg.rpc_url), slot)?
            }
            Commands::BlockTransactions { slot } => {
                commands::block::get_block(&rpc::get_client(&cfg.rpc_url), slot)?
            } // Currently returning everything, verify if specialized needed
            Commands::BlockRewards { slot } => {
                commands::block::get_block(&rpc::get_client(&cfg.rpc_url), slot)?
            } // Currently returning everything

            // Validator
            Commands::ValidatorList => {
                commands::validator::get_validators(&rpc::get_client(&cfg.rpc_url))?
            }
            Commands::ValidatorInfo { identity: _ } => {
                println!("Validator detailed info coming soon.")
            }

            // Token
            Commands::TokenMint { address } => {
                commands::token::get_mint_info(&rpc::get_client(&cfg.rpc_url), &address)?
            }
            Commands::TokenHolders { address } => {
                commands::token::get_holders(&rpc::get_client(&cfg.rpc_url), &address)?
            }

            // Stake
            Commands::StakeAccountInfo { address } => {
                commands::stake::get_stake_account(&rpc::get_client(&cfg.rpc_url), &address)?
            }

            // Program
            Commands::ProgramInfo { address } => {
                commands::program::get_info(&rpc::get_client(&cfg.rpc_url), &address)?
            }
            Commands::ProgramAccounts { address } => {
                commands::program::get_accounts(&rpc::get_client(&cfg.rpc_url), &address)?
            }
        }
    } else {
        // This path is hit if no subcommand is provided but no help/version flag triggered (e.g. empty execution)
        // Clap arg_required_else_help usually handles this, but we cover it safely.
    }
    Ok(())
}
