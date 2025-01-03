use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use inquire::{Password, PasswordDisplayMode};
use std::fs::{create_dir_all, read, read_to_string, write};

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
    /// Exports Bothan's hex-encoded monitoring private key
    Export {},
    /// Imports a hex-encoded monitoring private key
    Import {
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
            KeySubCommand::Export {} => export_key(&app_config),
            KeySubCommand::Import { override_ } => import_key(&app_config, *override_),
            KeySubCommand::Display { .. } => display_public_key(&app_config),
        }
    }
}

fn generate_key(config: &AppConfig, override_: bool) -> anyhow::Result<()> {
    if config.monitoring.path.is_file() && !override_ {
        return Err(anyhow!(
            "Monitoring key already exists. Use --override or -o to overwrite."
        ));
    }

    let signer = Signer::random();

    if let Some(parent) = config.monitoring.path.parent() {
        create_dir_all(parent).with_context(|| "Failed to create monitoring key directory")?;
    }

    write(&config.monitoring.path, signer.to_hex().as_bytes())
        .with_context(|| "Failed to write monitoring key file")?;

    Ok(())
}

fn export_key(config: &AppConfig) -> anyhow::Result<()> {
    let pkb =
        read(&config.monitoring.path).with_context(|| "Failed to read monitoring key file")?;
    let pk = String::from_utf8(pkb).with_context(|| "Failed to parse monitoring key file")?;
    println!("Private Key");
    println!("{}", pk);
    Ok(())
}

fn import_key(config: &AppConfig, override_: bool) -> anyhow::Result<()> {
    if config.monitoring.path.is_file() && !override_ {
        return Err(anyhow!(
            "Monitoring key already exists. Use --override or -o to overwrite."
        ));
    }

    let inquiry = Password::new("Enter the monitoring private key")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation();

    let key = inquiry.prompt().context("Failed to read private key")?;
    Signer::from_hex(&key).context("Private key is not valid")?;
    write(&config.monitoring.path, key).with_context(|| "Failed to write monitoring key file")?;

    println!("Monitoring key imported successfully");
    Ok(())
}

fn display_public_key(config: &AppConfig) -> anyhow::Result<()> {
    let file = read_to_string(&config.monitoring.path).expect("Failed to read monitoring key");
    let signer = Signer::from_hex(&file).expect("Failed to parse monitoring key");
    println!("Public Key");
    println!("{}", signer.public_key());

    Ok(())
}
