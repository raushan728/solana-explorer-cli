use anyhow::Result;
use chrono;
use colored::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    EncodedTransaction, UiMessage, UiTransactionEncoding, option_serializer::OptionSerializer,
};
use std::str::FromStr;

/// Fetch and display detailed transaction information.
///
/// This function retrieves the transaction details from the Solana cluster using the provided signature.
/// It displays:
/// - Signature, Slot, and Block Time (Local)
/// - Execution Status (Success/Failure)
/// - Compute Units Consumed
/// - Fee (in lamports)
/// - Transaction Logs
/// - Invoked Program IDs
pub fn get_details(client: &RpcClient, sig_str: &str) -> Result<()> {
    let signature =
        Signature::from_str(sig_str).map_err(|_| anyhow::anyhow!("Invalid Signature format"))?;

    // Fetch transaction with JSON encoding to parse instructions/accounts easily
    let tx = client.get_transaction_with_config(
        &signature,
        solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(solana_sdk::commitment_config::CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        },
    )?;

    println!("\n{}", "--- Transaction Details ---".bold().cyan());
    println!("{:<20} : {}", "Signature", sig_str.yellow());
    println!("{:<20} : {}", "Slot", tx.slot);

    // Fetch Block Time
    if let Some(time) = tx.block_time {
        let dt = chrono::DateTime::from_timestamp(time, 0).unwrap_or_default();
        println!(
            "{:<20} : {}",
            "Timestamp",
            dt.format("%Y-%m-%d %H:%M:%S %Z").to_string().cyan()
        );
    } else {
        println!("{:<20} : {}", "Timestamp", "Unavailable".red());
    }

    // Parse Transaction Meta
    if let Some(meta) = &tx.transaction.meta {
        // Handle OptionSerializer for Compute Units
        let units = match meta.compute_units_consumed {
            OptionSerializer::Some(val) => val,
            OptionSerializer::None => 0,
            OptionSerializer::Skip => 0,
        };

        let fee = meta.fee;

        println!("{:<20} : {} units", "Compute Units", units);
        println!("{:<20} : {} lamports", "Fee", fee);

        // Status check
        match &meta.err {
            Some(err) => println!("{:<20} : {} ({:?})", "Status", "FAILED".red().bold(), err),
            None => println!("{:<20} : {}", "Status", "SUCCESS".green().bold()),
        }

        // Display Logs
        if let OptionSerializer::Some(logs) = &meta.log_messages {
            println!("\n{}", "--- Transaction Logs ---".bold().cyan());
            for (i, log) in logs.iter().enumerate() {
                println!("[{:02}] {}", i, log);
            }
        }

        // Display Inner Instructions
        if let OptionSerializer::Some(inner_instructions) = &meta.inner_instructions {
            if !inner_instructions.is_empty() {
                println!("\n{}", "--- Inner Instructions ---".bold().cyan());
                for (idx, ix_list) in inner_instructions.iter().enumerate() {
                    println!("  Program Instruction {}", idx);
                    for (inner_idx, inner_ix) in ix_list.instructions.iter().enumerate() {
                        // Handle UiInstruction Enum (Compiled vs parsed)
                        match inner_ix {
                            solana_transaction_status::UiInstruction::Compiled(compiled) => {
                                // Program ID is an index in account keys, hard to resolve here without passing account keys map down.
                                // simpler to print index
                                println!("    [{:02}] Program Index: {}", inner_idx, compiled.program_id_index);
                            }
                            solana_transaction_status::UiInstruction::Parsed(parsed) => {
                                match parsed {
                                    solana_transaction_status::UiParsedInstruction::Parsed(p) => {
                                        println!("    [{:02}] Program: {}", inner_idx, p.program.purple());
                                    }
                                    solana_transaction_status::UiParsedInstruction::PartiallyDecoded(pd) => {
                                        println!("    [{:02}] Program ID: {}", inner_idx, pd.program_id.purple());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Extract and Display Program IDs involved in the transaction
    if let EncodedTransaction::Json(ui_tx) = tx.transaction.transaction {
        if let UiMessage::Raw(msg) = ui_tx.message {
            println!("\n{}", "--- Involved Programs ---".bold().cyan());
            let account_keys = msg.account_keys;

            // Collect unique program IDs from instructions
            let mut programs = Vec::new();
            for ix in msg.instructions {
                let program_idx = ix.program_id_index as usize;
                if program_idx < account_keys.len() {
                    let prog_id = &account_keys[program_idx];
                    if !programs.contains(prog_id) {
                        programs.push(prog_id.clone());
                    }
                }
            }

            for prog in programs {
                println!("- {}", prog.magenta());
            }
        }
    }

    println!("{}\n", "---------------------------".bold().cyan());

    Ok(())
}

pub fn get_logs(client: &RpcClient, sig_str: &str) -> Result<()> {
    // Similar to details but only prints logs
    let signature = Signature::from_str(sig_str).map_err(|_| anyhow::anyhow!("Invalid Sig"))?;
    let tx = client.get_transaction_with_config(
        &signature,
        solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(solana_sdk::commitment_config::CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        },
    )?;

    if let Some(meta) = tx.transaction.meta {
        if let OptionSerializer::Some(logs) = meta.log_messages {
            println!("\n{}", "--- Transaction Logs ---".bold().cyan());
            for (i, log) in logs.iter().enumerate() {
                println!("[{:02}] {}", i, log);
            }
        } else {
            println!("No logs found.");
        }
    }
    Ok(())
}
