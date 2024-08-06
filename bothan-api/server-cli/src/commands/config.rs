use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use bothan_api::config::AppConfig;

use crate::commands::CliExec;

#[derive(Parser)]
pub struct ConfigCli {
    #[command(subcommand)]
    subcommands: ConfigSubCommands,
}

#[derive(Subcommand)]
enum ConfigSubCommands {
    Init {
        /// The path to where to initialize the configuration file (defaults to ./config.toml).
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[async_trait::async_trait]
impl CliExec for ConfigCli {
    async fn run(&self) {
        match &self.subcommands {
            ConfigSubCommands::Init { path } => {
                let config_path = match path {
                    Some(p) => p.clone(),
                    None => PathBuf::from("./config.toml"),
                };

                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent).expect("Failed to create config directory");
                }

                let app_config = AppConfig::default();
                let string_cfg = toml::to_string(&app_config).expect("Failed to serialize config");

                fs::write(&config_path, string_cfg).expect("Failed to write config");
                println!("Initialized default config at: {:?}", config_path);
            }
        }
    }
}
