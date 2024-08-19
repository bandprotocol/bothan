use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};

use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;

#[derive(Parser)]
pub struct ConfigCli {
    #[command(subcommand)]
    subcommand: ConfigSubCommand,
}

#[derive(Subcommand)]
enum ConfigSubCommand {
    Init {
        /// The path to where to initialize the configuration file (defaults to ./config.toml).
        #[arg(short, long)]
        #[clap(default_value = "./config.toml")]
        path: PathBuf,
    },
}

impl ConfigCli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            ConfigSubCommand::Init { path } => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create parent directories for {:?}", path)
                    })?;
                }

                let mut app_config = AppConfig::default();
                app_config.manager.crypto.source = CryptoSourceConfigs::with_default_sources();
                let string_cfg =
                    toml::to_string(&app_config).with_context(|| "Failed to serialize config")?;

                fs::write(path, string_cfg).with_context(|| "Failed to write config")?;
                println!("Initialized default config at: {:?}", path);
                Ok(())
            }
        }
    }
}
