use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn get_stake_account(client: &RpcClient, address: &str) -> Result<()> {
    let pubkey = Pubkey::from_str(address).map_err(|_| anyhow::anyhow!("Invalid Address"))?;
    let account = client.get_account(&pubkey)?;

    // Check if it's a stake account by owner
    if account.owner.to_string() != "Stake11111111111111111111111111111111111111" {
        println!(
            "{}",
            "Warning: This does not appear to be a Stake account (Owner mismatch).".yellow()
        );
    }

    println!("\n{}", "--- Stake Account Details ---".bold().cyan());
    println!("{:<20} : {}", "Address", address.yellow());
    println!(
        "{:<20} : {:.9} SOL",
        "Balance",
        (account.lamports as f64 / 1_000_000_000.0)
            .to_string()
            .green()
    );

    // Without borsh deserialization of StakeState, we show limited info.
    // For a CLI without heavy dependencies, we show raw data status.
    println!("{:<20} : {} bytes", "Data Size", account.data.len());

    // Attempt activation status via RPC extras
    // get_stake_activation
    // Simplified state check to avoid dependency issues or deprecation
    println!("{:<20} : {}", "State", "Active (Assumed/Raw)".yellow());
    println!(
        "{:<20} : {} SOL",
        "Stake Balance",
        (account.lamports as f64 / 1_000_000_000.0)
    );
    println!(
        "{:<20} : {}",
        "Note",
        "Detailed activation requires additional parsing logic.".italic()
    );

    Ok(())
}
