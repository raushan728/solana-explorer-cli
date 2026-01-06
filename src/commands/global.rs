use anyhow::Result;
use colored::*;
use solana_client::rpc_client::RpcClient;

pub fn get_supply(client: &RpcClient) -> Result<()> {
    let supply = client
        .supply_with_commitment(solana_sdk::commitment_config::CommitmentConfig::finalized())?
        .value;
    println!("\n{}", "--- SOL Supply ---".bold().cyan());
    println!(
        "{:<25} : {} SOL",
        "Total Supply",
        (supply.total as f64 / 1_000_000_000.0).to_string().green()
    );
    println!(
        "{:<25} : {} SOL",
        "Circulating",
        (supply.circulating as f64 / 1_000_000_000.0)
            .to_string()
            .green()
    );
    println!(
        "{:<25} : {} SOL",
        "Non-Circulating",
        (supply.non_circulating as f64 / 1_000_000_000.0)
            .to_string()
            .yellow()
    );
    Ok(())
}

pub fn get_inflation(client: &RpcClient) -> Result<()> {
    let governor = client.get_inflation_governor()?;
    // We can also get current rate
    let rate = client.get_inflation_rate()?;

    println!("\n{}", "--- Inflation Status ---".bold().cyan());
    println!("{:<25} : {:.2}%", "Total Rate", rate.total * 100.0);
    println!("{:<25} : {:.2}%", "Validator Rate", rate.validator * 100.0);
    println!(
        "{:<25} : {:.2}%",
        "Foundation Rate",
        rate.foundation * 100.0
    );
    println!(
        "{:<25} : {:.2}%",
        "Terminal Rate",
        governor.terminal * 100.0
    );
    Ok(())
}

pub fn get_tps(client: &RpcClient) -> Result<()> {
    let samples = client.get_recent_performance_samples(Some(5))?;

    println!(
        "\n{}",
        "--- Network Performance (Avg last 5 samples) ---"
            .bold()
            .cyan()
    );

    if let Some(sample) = samples.first() {
        // num_transactions / sample_period_secs
        let tps = sample.num_transactions as f64 / sample.sample_period_secs as f64;
        println!(
            "{:<25} : {:.2} TPS",
            "Current TPS",
            tps.to_string().green().bold()
        );
        println!(
            "{:<25} : {} slots",
            "Sample Period", sample.sample_period_secs
        );
        println!(
            "{:<25} : {} txs",
            "Transactions in Window", sample.num_transactions
        );
    } else {
        println!("No performance samples available.");
    }

    Ok(())
}
