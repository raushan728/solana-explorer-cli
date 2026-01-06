use anyhow::Result;
use colored::*;
use prettytable::{Cell, Row, Table};
use solana_client::rpc_client::RpcClient;

pub fn get_validators(client: &RpcClient) -> Result<()> {
    println!("Fetching validator set...");
    let vote_accounts = client.get_vote_accounts()?;

    println!(
        "\n{}",
        format!(
            "--- Active Validators ({}) ---",
            vote_accounts.current.len()
        )
        .bold()
        .cyan()
    );

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Node Pubkey").style_spec("Fm"),
        Cell::new("Vote Pubkey").style_spec("Fy"),
        Cell::new("Commission").style_spec("Fc"),
        Cell::new("Activated Stake").style_spec("Fg"),
    ]));

    // Sort by stake details? They come somewhat sorted.
    // Take top 20
    for vote in vote_accounts.current.iter().take(20) {
        table.add_row(Row::new(vec![
            Cell::new(&vote.node_pubkey),
            Cell::new(&vote.vote_pubkey),
            Cell::new(&format!("{}%", vote.commission)),
            Cell::new(&format!(
                "{:.2} SOL",
                vote.activated_stake as f64 / 1_000_000_000.0
            )),
        ]));
    }

    table.printstd();
    if vote_accounts.current.len() > 20 {
        println!("... and {} more.", vote_accounts.current.len() - 20);
    }

    Ok(())
}
