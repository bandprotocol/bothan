use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    pub async fn run(&self) {
        match &self.subcommand {
            ConfigSubCommand::Init { path } => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).expect("Failed to create config directory");
                }

                let app_config = AppConfig::default();
                let string_cfg = toml::to_string(&app_config).expect("Failed to serialize config");

                fs::write(path, string_cfg).expect("Failed to write config");
                println!("Initialized default config at: {:?}", path);
            }
        }
    }
}
