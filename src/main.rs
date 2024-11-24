mod cli;
use alloy::providers::{Provider, ProviderBuilder};
use clap::Parser;
use cli::{BlockCommands, Commands};
use eyre::Result;

// const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/xGVIcO70-O7Bd8_DSfR2yomVI2hrvKZE";
const RPC_URL: &str = "https://polygon-amoy.g.alchemy.com/v2/xGVIcO70-O7Bd8_DSfR2yomVI2hrvKZE";

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_http(RPC_URL.parse()?);

    let accounts = provider.get_accounts().await?;
    println!("{accounts:?}");

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
        _ => {}
    };

    Ok(())
}
