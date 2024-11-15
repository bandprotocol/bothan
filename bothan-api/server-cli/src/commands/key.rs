use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};

use bothan_api::config::AppConfig;
use bothan_core::monitoring::Signer;

#[derive(Parser)]
pub struct KeyCli {
    #[command(subcommand)]
    subcommand: KeySubCommand,
}

#[derive(Subcommand)]
enum KeySubCommand {
    /// Generates a new monitoring key
    Generate {
        /// Whether to override the existing key.
        #[arg(short, long = "override")]
        override_: bool,
    },
    /// Exports the monitoring key
    Export {
        /// The path to export the key to.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Imports a monitoring key
    Import {
        /// The path to import the key from.
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Whether to override the existing key.
        #[arg(short, long = "override")]
        override_: bool,
    },
    /// Displays the monitoring public key
    Display {},
}

impl KeyCli {
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        match &self.subcommand {
            KeySubCommand::Generate { override_ } => generate_key(&app_config, *override_),
            KeySubCommand::Export { path } => export_key(&app_config, path),
            KeySubCommand::Import { path, override_ } => import_key(&app_config, path, *override_),
            KeySubCommand::Display { .. } => display_public_key(&app_config),
        }
    }
}

fn generate_key(config: &AppConfig, override_: bool) -> anyhow::Result<()> {
    if config.monitoring.path.is_file() && !override_ {
        return Err(anyhow!(
            "Monitoring key file already exists. Use --override to overwrite."
        ));
    }

    let signer = Signer::random();

    if let Some(parent) = config.monitoring.path.parent() {
        create_dir_all(parent).with_context(|| "Failed to create monitoring key directory")?;
    }

    let mut file = File::create(&config.monitoring.path)
        .with_context(|| "Failed to create monitoring key file")?;

    file.write(signer.to_hex().as_bytes())
        .with_context(|| "Failed to write monitoring key file")?;

    Ok(())
}

fn export_key(config: &AppConfig, path: &Option<PathBuf>) -> anyhow::Result<()> {
    let destination = match path {
        Some(p) => p,
        None => &std::env::current_dir()?.join("key.txt"),
    };

    fs::copy(&config.monitoring.path, destination)
        .with_context(|| format!("Failed to copy monitoring key to {:?}", destination))?;

    Ok(())
}

fn import_key(config: &AppConfig, path: &Option<PathBuf>, override_: bool) -> anyhow::Result<()> {
    if config.monitoring.path.is_file() && !override_ {
        return Err(anyhow!(
            "Monitoring key file already exists. Use --override to overwrite."
        ));
    }

    let file = match path {
        Some(p) => p,
        None => &std::env::current_dir()?.join("key.txt"),
    };

    fs::copy(file, &config.monitoring.path)
        .with_context(|| format!("Failed to copy monitoring key from {:?}", file))?;

    Ok(())
}

fn display_public_key(config: &AppConfig) -> anyhow::Result<()> {
    let file = fs::read_to_string(&config.monitoring.path)
        .with_context(|| "Failed to read monitoring key file")?;
    let signer = Signer::from_hex(&file).with_context(|| "Failed to parse monitoring key")?;

    println!("public key: {}", signer.public_key());
    Ok(())
}
