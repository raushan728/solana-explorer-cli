use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;

pub fn get_status(client: &RpcClient) -> Result<()> {
    let epoch_info = client.get_epoch_info()?;
    let version = client.get_version()?;
    let block_height = client.get_block_height()?;

    println!(
        "\n{}",
        "--- Solana Network Status ---".bold().bright_magenta()
    );
    println!("{:<20} : {}", "Cluster Version", version.solana_core.cyan());
    println!(
        "{:<20} : {}",
        "Current Epoch",
        epoch_info.epoch.to_string().yellow()
    );
    println!(
        "{:<20} : {}%",
        "Epoch Progress",
        (epoch_info.slot_index as f64 / epoch_info.slots_in_epoch as f64 * 100.0).round()
    );
    println!(
        "{:<20} : {}",
        "Current Slot",
        epoch_info.absolute_slot.to_string().green()
    );
    println!(
        "{:<20} : {}",
        "Block Height",
        block_height.to_string().green()
    );
    println!(
        "{}\n",
        "-----------------------------".bold().bright_magenta()
    );

    Ok(())
}
