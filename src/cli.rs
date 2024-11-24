use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commands related to blocks
    Block {
        #[command(subcommand)]
        command: Option<BlockCommands>,
    },
}

#[derive(Subcommand)]
pub enum BlockCommands {
    /// Get latest block number
    Number,
}
