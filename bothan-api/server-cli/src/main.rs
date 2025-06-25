//! # Bothan API Server CLI
//!
//! This is the command-line interface for the Bothan API Server, providing
//! various commands for server management, configuration, and data access.
//!
//! ## Overview
//!
//! The CLI provides the following main commands:
//! - **config**: Manage server configuration files
//! - **key**: Handle API keys and authentication
//! - **query**: Query cryptocurrency data and market information
//! - **request**: Make direct API requests to the server
//! - **start**: Start the Bothan API server
//!
//! ## Usage
//!
//! ```bash
//! # Start the server
//! bothan start
//!
//! # Initialize configuration
//! bothan config init
//!
//! # Query market data
//! bothan query price BTC/USD
//!
//! # Use custom config file
//! bothan --config /path/to/config.toml start
//! ```
//!
//! ## Configuration
//!
//! The CLI automatically looks for configuration in `~/.bothan/config.toml`.
//! You can specify a custom config file using the `--config` flag.
//!
//! ## Logging
//!
//! The CLI configures comprehensive logging for all Bothan components,
//! including individual exchange integrations and core services.

use std::path::PathBuf;
use std::str::FromStr;

use bothan_api::config::AppConfig;
use bothan_api::config::log::LogLevel;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Directive;

use crate::commands::config::ConfigCli;
use crate::commands::key::KeyCli;
use crate::commands::query::QueryCli;
use crate::commands::request::RequestCli;
use crate::commands::start::StartCli;
use crate::helper::Exitable;

mod commands;
mod helper;

/// Main CLI application structure with global configuration options.
///
/// This struct defines the top-level command-line interface with
/// subcommands for different server operations and global options
/// like configuration file path.
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to the configuration file (optional, defaults to ~/.bothan/config.toml)
    #[arg(long, global = true)]
    config: Option<PathBuf>,
}

/// Available CLI subcommands for server management and data access.
///
/// Each command provides specific functionality for interacting with
/// the Bothan API server and managing its configuration.
#[derive(Subcommand)]
enum Command {
    /// Configuration command for the bothan-api server
    Config(ConfigCli),
    /// Key command for the bothan-api server
    Key(KeyCli),
    /// Query command for the bothan-api server-cli
    Query(QueryCli),
    /// Request command for the bothan-api server
    Request(RequestCli),
    /// Starts the bothan-api server
    Start(StartCli),
}

/// Main entry point for the Bothan API Server CLI.
///
/// This function handles command-line parsing, configuration loading,
/// logging setup, and command execution. It provides a unified interface
/// for all server management operations.
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
        .add_directive(create_directive("bothan_lib", core_log_lvl))
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
            Command::Query(query_cli) => query_cli.run(app_config).await,
            Command::Request(request_cli) => request_cli.run(app_config).await,
            Command::Start(start_cli) => start_cli.run(app_config).await,
        };
        res.exit_on_err(1);
    }
}

/// Creates a logging directive for a specific module and log level.
///
/// This function constructs a tracing directive string that controls
/// the logging level for individual Bothan components.
///
/// # Arguments
///
/// * `module` - The module name to create a directive for
/// * `log_level` - The desired logging level
///
/// # Returns
///
/// Returns a `Directive` that can be added to the logging filter.
///
/// # Example
///
/// ```rust,no_run
/// use bothan_api::config::log::LogLevel;
///
/// let directive = create_directive("bothan_api", &LogLevel::Info);
/// ```
fn create_directive(module: &str, log_level: &LogLevel) -> Directive {
    Directive::from_str(&format!("{module}={log_level}")).expect("Failed to create directive}")
}

/// Returns the Bothan home directory path.
///
/// This function constructs the path to the Bothan configuration
/// directory, which is typically located at `~/.bothan/`.
///
/// # Returns
///
/// Returns a `PathBuf` pointing to the Bothan home directory.
///
/// # Panics
///
/// Panics if the user's home directory cannot be determined.
///
/// # Example
///
/// ```rust,no_run
/// let home_dir = bothan_home_dir();
/// println!("Bothan home: {:?}", home_dir);
/// // Output: Bothan home: "/home/user/.bothan"
/// ```
pub fn bothan_home_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}
