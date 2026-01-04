use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn get_info(client: &RpcClient, address_str: &str) -> Result<()> {
    let pubkey = Pubkey::from_str(address_str)
        .map_err(|_| anyhow::anyhow!("Invalid Solana address format"))?;

    let account = client.get_account(&pubkey)?;
    let balance = client.get_balance(&pubkey)?;

    println!("\n{}", "--- Account Information ---".bold().bright_blue());
    println!("{:<20} : {}", "Address", address_str.yellow());
    println!(
        "{:<20} : {} SOL",
        "Balance",
        (balance as f64 / 1_000_000_000.0)
            .to_string()
            .green()
            .bold()
    );
    println!("{:<20} : {} lamports", "Lamports", balance);
    println!(
        "{:<20} : {}",
        "Owner Program",
        account.owner.to_string().cyan()
    );
    println!(
        "{:<20} : {}",
        "Executable",
        if account.executable {
            "Yes".green()
        } else {
            "No".red()
        }
    );
    println!("{:<20} : {}", "Data Size", account.data.len());
    println!("{}\n", "---------------------------".bold().bright_blue());

    Ok(())
}
