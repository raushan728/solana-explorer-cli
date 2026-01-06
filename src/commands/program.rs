use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn get_info(client: &RpcClient, prog_id: &str) -> Result<()> {
    let pubkey = Pubkey::from_str(prog_id).map_err(|_| anyhow::anyhow!("Invalid Program ID"))?;
    let account = client.get_account(&pubkey)?;

    if !account.executable {
        println!("{}", "Warning: Account is not marked executable!".yellow());
    }

    println!("\n{}", "--- Program Details ---".bold().cyan());
    println!("{:<20} : {}", "Program ID", prog_id.yellow());
    println!("{:<20} : {}", "Owner", account.owner);
    println!("{:<20} : {} bytes", "Data Size", account.data.len());
    println!(
        "{:<20} : {} SOL",
        "Balance",
        account.lamports as f64 / 1_000_000_000.0
    );
    Ok(())
}

pub fn get_accounts(client: &RpcClient, prog_id: &str) -> Result<()> {
    let pubkey = Pubkey::from_str(prog_id).map_err(|_| anyhow::anyhow!("Invalid Program ID"))?;
    // This is a heavy call
    println!("Fetching accounts owned by program (Limit 20)...");

    // We cannot easily limit via RPC without config, but get_program_accounts is standard.
    // We'll trust user uses this on reasonable programs.
    // Or we use `get_program_accounts_with_config` and filters.
    // For now, basic call.
    let accounts = client.get_program_accounts(&pubkey)?;

    println!(
        "\n{}",
        format!("--- Owned Accounts ({}) ---", accounts.len())
            .bold()
            .cyan()
    );
    for (pk, acc) in accounts.iter().take(20) {
        println!("{:<44} | {} lamports", pk.to_string(), acc.lamports);
    }
    if accounts.len() > 20 {
        println!("... and {} more.", accounts.len() - 20);
    }
    Ok(())
}
