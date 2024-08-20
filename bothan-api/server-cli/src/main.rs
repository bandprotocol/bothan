use clap::{Parser, Subcommand};

use crate::commands::config::ConfigCli;
use crate::commands::start::StartCli;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Configuration command for the bothan-api server
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

    if let Some(command) = &cli.command {
        if let Err(e) = match command {
            Command::Config(config_cli) => config_cli.run().await,
            Command::Start(start_cli) => start_cli.run().await,
        } {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
