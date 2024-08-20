use clap::{Parser, Subcommand};

use crate::commands::config::ConfigCli;
use crate::commands::start::StartCli;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Config
    Config(ConfigCli),
    /// Starts the bothan-api server
    Start(StartCli),
}

#[tokio::main]
async fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let r = match &cli.command {
        Some(Command::Config(cli)) => cli.run().await,
        Some(Command::Start(cli)) => cli.run().await,
        None => Ok(()), // Placeholder
    };

    if let Err(e) = r {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}