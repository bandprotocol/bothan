use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use tracing_subscriber::filter::Directive;
use tracing_subscriber::EnvFilter;

use bothan_api::config::log::LogLevel;
use bothan_api::config::AppConfig;

use crate::commands::config::ConfigCli;
use crate::commands::key::KeyCli;
use crate::commands::request::RequestCli;
use crate::commands::start::StartCli;
use crate::helper::Exitable;

mod commands;
mod helper;

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
    /// Key command for the bothan-api server
    Key(KeyCli),
    /// Request command for the bothan-api server
    Request(RequestCli),
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

    let config_path = &cli.config.unwrap_or(bothan_home_dir().join("config.toml"));

    let app_config = if config_path.is_file() {
        AppConfig::from(config_path).expect(
            "Failed to load config. Try deleting the config file and running 'config init'.",
        )
    } else {
        AppConfig::default()
    };

    let log_lvl = &app_config.log.log_level;
    let core_log_lvl = &app_config.log.core_log_level;
    let src_log_lvl = &app_config.log.source_log_level;

    let filter = EnvFilter::new(format!("bothan={log_lvl}"))
        .add_directive(create_directive("bothan_api", log_lvl))
        .add_directive(create_directive("bothan_core", core_log_lvl))
        .add_directive(create_directive("bothan_binance", src_log_lvl))
        .add_directive(create_directive("bothan_bybit", src_log_lvl))
        .add_directive(create_directive("bothan_coinbase", src_log_lvl))
        .add_directive(create_directive("bothan_coingecko", src_log_lvl))
        .add_directive(create_directive("bothan_coinmarketcap", src_log_lvl))
        .add_directive(create_directive("bothan_cryptocompare", src_log_lvl))
        .add_directive(create_directive("bothan_htx", src_log_lvl))
        .add_directive(create_directive("bothan_kraken", src_log_lvl))
        .add_directive(create_directive("bothan_okx", src_log_lvl));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Some(command) = &cli.command {
        let res = match command {
            Command::Config(config_cli) => config_cli.run().await,
            Command::Key(key_cli) => key_cli.run(app_config).await,
            Command::Request(request_cli) => request_cli.run(app_config).await,
            Command::Start(start_cli) => start_cli.run(app_config).await,
        };
        res.exit_on_err(1);
    }
}

fn create_directive(module: &str, log_level: &LogLevel) -> Directive {
    Directive::from_str(&format!("{module}={log_level}")).expect("Failed to create directive}")
}

pub fn bothan_home_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}
