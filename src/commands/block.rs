use anyhow::Result;
use colored::*;
use prettytable::{Cell, Row, Table};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};

/// Fetch and display detailed block information.
pub fn get_block(client: &RpcClient, slot: u64) -> Result<()> {
    println!("Fetching block {}...", slot);

    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Signatures),
        rewards: Some(true),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),
    };

    let block = client.get_block_with_config(slot, config)?;

    println!("\n{}", "--- Block Details ---".bold().cyan());
    println!("{:<20} : {}", "Slot", slot.to_string().green());
    println!("{:<20} : {}", "Blockhash", block.blockhash.yellow());
    println!(
        "{:<20} : {}",
        "Previous Blockhash", block.previous_blockhash
    );

    if let Some(time) = block.block_time {
        let dt = chrono::DateTime::from_timestamp(time, 0).unwrap_or_default();
        println!("{:<20} : {}", "Block Time", dt.to_rfc2822());
    }

    println!("{:<20} : {}", "Parent Slot", block.parent_slot);

    if let Some(rewards) = &block.rewards {
        println!("{:<20} : {} entries", "Rewards", rewards.len());
    }

    if let Some(signatures) = &block.signatures {
        println!(
            "\n{}",
            format!("--- Transactions ({}) ---", signatures.len())
                .bold()
                .cyan()
        );

        // Show first 10 txs
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Index").style_spec("Fm"),
            Cell::new("Signature").style_spec("Fy"),
        ]));

        for (i, sig) in signatures.iter().take(10).enumerate() {
            table.add_row(Row::new(vec![Cell::new(&i.to_string()), Cell::new(sig)]));
        }
        table.printstd();

        if signatures.len() > 10 {
            println!("... and {} more transactions.", signatures.len() - 10);
        }
    }

    Ok(())
}
