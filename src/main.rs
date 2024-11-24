mod cli;
mod consts;
mod transaction;
use alloy::providers::*;
use clap::Parser;
use cli::{BlockCommands, Commands};
use consts::RPC_URL;
use eyre::Result;
use transaction::my_send_transaction;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_http(RPC_URL.parse()?);

    let cli = cli::Cli::parse();

    match &cli.command {
        Some(Commands::Block {
            command: Some(BlockCommands::Number),
        }) => {
            // Get latest block number.
            let latest_block = provider.get_block_number().await?;

            // Print the block number.
            println!("Latest block number: {latest_block}");
        }
        Some(Commands::Transaction) => {
            my_send_transaction().await?;
        }
        _ => {}
    };

    Ok(())
}
