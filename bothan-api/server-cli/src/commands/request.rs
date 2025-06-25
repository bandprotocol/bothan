//! Bothan CLI request subcommand module.
//!
//! Provides commands to make direct gRPC requests to the Bothan API server:
//! - Get server info
//! - Update the registry
//! - Push monitoring records
//! - Get prices for signal ids

use anyhow::Context;
use bothan_api::config::AppConfig;
use bothan_client::client::GrpcClient;
use clap::{Parser, Subcommand};

#[derive(Parser)]
/// CLI arguments for the `request` command.
pub struct RequestCli {
    /// Optional URI to connect to the Bothan API server.
    #[arg(long)]
    uri: Option<String>,
    /// The subcommand to execute.
    #[command(subcommand)]
    subcommand: RequestSubCommand,
}

#[derive(Subcommand)]
/// Supported request subcommands for the Bothan CLI.
enum RequestSubCommand {
    /// Get information about the server.
    GetInfo,
    /// Update the registry of the server.
    UpdateRegistry {
        /// The IPFS hash of the registry to update.
        ipfs_hash: String,
        /// The version of the registry to update.
        version: String,
    },
    /// Push monitoring records to the server.
    PushMonitoringRecords {
        /// The UUID of the monitoring records.
        uuid: String,
        /// The transaction hash of the monitoring records.
        tx_hash: String,
        /// The list of signal ids to push.
        signal_ids: Vec<String>,
    },
    /// Get prices for a list of signal ids.
    GetPrices {
        /// The list of signal ids to get prices for.
        signal_ids: Vec<String>,
    },
}

impl RequestCli {
    /// Runs the request command and executes the selected subcommand.
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        let uri = self
            .uri
            .clone()
            .unwrap_or(format!("https://{}", &app_config.grpc.addr));
        let client = match GrpcClient::connect(&uri).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to connect to server: {:#?}", e);
                std::process::exit(1);
            }
        };

        match &self.subcommand {
            RequestSubCommand::GetInfo => {
                let info = client
                    .get_info()
                    .await
                    .with_context(|| "Failed to get info")?;
                println!("{:#?}", info);
            }
            RequestSubCommand::UpdateRegistry { ipfs_hash, version } => {
                client
                    .update_registry(ipfs_hash, version)
                    .await
                    .with_context(|| "Failed to update registry")?;
                println!("Registry updated");
            }
            RequestSubCommand::PushMonitoringRecords {
                uuid,
                tx_hash,
                signal_ids,
            } => {
                let ids = signal_ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                client
                    .push_monitoring_records(uuid, tx_hash, &ids)
                    .await
                    .with_context(|| "Failed to push monitoring records")?;
                println!("Monitoring records pushed");
            }
            RequestSubCommand::GetPrices { signal_ids } => {
                let ids = signal_ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                let prices = client
                    .get_prices(&ids)
                    .await
                    .with_context(|| "Failed to get prices")?;
                println!("{:#?}", prices);
            }
        }
        Ok(())
    }
}
