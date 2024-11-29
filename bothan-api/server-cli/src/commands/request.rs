use anyhow::Context;
use clap::{Parser, Subcommand};

use bothan_api::config::AppConfig;
use bothan_client::client::GrpcClient;

#[derive(Parser)]
pub struct RequestCli {
    #[arg(long)]
    uri: Option<String>,

    #[command(subcommand)]
    subcommand: RequestSubCommand,
}

#[derive(Subcommand)]
enum RequestSubCommand {
    /// Get information about the server
    GetInfo,
    /// Update the registry of the server
    UpdateRegistry {
        /// The IPFS hash of the registry to update
        ipfs_hash: String,
        /// The version of the registry to update
        version: String,
    },
    /// Push monitoring records to the server
    PushMonitoringRecords {
        /// The UUID of the monitoring records
        uuid: String,
        /// The transaction hash of the monitoring records
        tx_hash: String,
    },
    /// Get prices for a list of signal ids
    GetPrices {
        /// The list of signal ids to get prices for
        signal_ids: Vec<String>,
    },
}

impl RequestCli {
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
            RequestSubCommand::PushMonitoringRecords { uuid, tx_hash } => {
                client
                    .push_monitoring_records(uuid, tx_hash)
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
