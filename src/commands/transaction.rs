use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::option_serializer::OptionSerializer;
use std::str::FromStr;

pub fn get_details(client: &RpcClient, sig_str: &str) -> Result<()> {
    let signature =
        Signature::from_str(sig_str).map_err(|_| anyhow::anyhow!("Invalid Signature format"))?;

    let tx = client.get_transaction_with_config(
        &signature,
        solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
            commitment: Some(solana_sdk::commitment_config::CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        },
    )?;

    println!("\n{}", "--- Transaction Details ---".bold().bright_green());
    println!("{:<20} : {}", "Signature", sig_str.yellow());
    println!("{:<20} : {}", "Slot", tx.slot);

    if let Some(meta) = &tx.transaction.meta {
        // OptionSerializer
        let units = match meta.compute_units_consumed {
            OptionSerializer::Some(val) => val,
            _ => 0,
        };

        let fee = meta.fee;

        println!("{:<20} : {} units", "Compute Units", units);
        println!("{:<20} : {} lamports", "Fee", fee);

        // Status check
        match &meta.err {
            Some(err) => println!("{:<20} : {} ({:?})", "Status", "FAILED".red().bold(), err),
            None => println!("{:<20} : {}", "Status", "SUCCESS".green().bold()),
        }
    }

    println!("{}\n", "---------------------------".bold().bright_green());

    Ok(())
}
