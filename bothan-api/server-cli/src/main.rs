use clap::{Parser, Subcommand};

use crate::commands::config::ConfigCli;
use crate::commands::start::StartCli;
use crate::commands::CliExec;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigCli),
    /// Starts the bothan-api server
    Start(StartCli),
}

#[async_trait::async_trait]
impl CliExec for Commands {
    async fn run(&self) {
        match self {
            Commands::Config(config) => config.run().await,
            Commands::Start(start) => start.run().await,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(cmd) => cmd.run().await,
        None => {}
    }
}
