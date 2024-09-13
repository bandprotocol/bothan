use std::path::PathBuf;

use clap::{Parser, Subcommand};

use bothan_api::config::AppConfig;

use crate::commands::config::ConfigCli;
use crate::commands::start::StartCli;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    // global args
    #[arg(long, global = true)]
    config: Option<PathBuf>,
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

    let config_path = cli
        .config
        .clone()
        .unwrap_or(bothan_home_dir().join("config.toml"));

    let app_config = match config_path.is_file() {
        true => AppConfig::from(config_path).expect("Failed to load config"),
        false => AppConfig::default(),
    };

    let log_level = &app_config.log.level;
    tracing_subscriber::fmt()
        .with_env_filter(format!(
            "bothan_core={log_level},bothan_api={log_level},bothan={log_level}"
        ))
        .init();

    if let Some(command) = &cli.command {
        match command {
            Command::Config(config_cli) => config_cli.run().await,
            Command::Start(start_cli) => start_cli.run(app_config).await,
        }
        .expect("Failed to run command");
    }
}

pub fn bothan_home_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}
