use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use tracing::info;

use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;

use crate::bothan_home_dir;

#[derive(Parser)]
pub struct ConfigCli {
    #[command(subcommand)]
    subcommand: ConfigSubCommand,
}

#[derive(Subcommand)]
enum ConfigSubCommand {
    /// Initialize the configuration file.
    Init {
        /// The path to where to initialize the configuration file (defaults to ./config.toml).
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Whether to override the existing configuration file.
        #[arg(short, long = "override")]
        override_: bool,
    },
}

impl ConfigCli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            ConfigSubCommand::Init { path, override_ } => {
                let config_path = match path {
                    Some(p) => p.clone(),
                    None => bothan_home_dir().join("config.toml"),
                };

                //check if the file already exists
                if config_path.exists() && !override_ {
                    return Err(anyhow!("Config file already exists at: {:?}", config_path));
                }

                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create parent directories for {:?}", path)
                    })?;
                }

                let mut app_config = AppConfig::default();
                app_config.manager.crypto.source = CryptoSourceConfigs::with_default_sources();
                let config_string =
                    toml::to_string(&app_config).with_context(|| "Failed to serialize config")?;

                fs::write(&config_path, config_string).with_context(|| "Failed to write config")?;
                info!("initialized default config at: {:?}", config_path);
                Ok(())
            }
        }
    }
}
