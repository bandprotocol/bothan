use anyhow::anyhow;
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
        let client = GrpcClient::connect(&uri)
            .await
            .map_err(|e| anyhow!("Failed to connect to server: {e}"))?;

        match &self.subcommand {
            RequestSubCommand::GetInfo => {
                let response = client.get_info().await?;
                println!("{:?}", response);
            }
            RequestSubCommand::UpdateRegistry { ipfs_hash, version } => {
                client.update_registry(ipfs_hash, version).await?;
            }
            RequestSubCommand::PushMonitoringRecords { uuid, tx_hash } => {
                client.push_monitoring_records(uuid, tx_hash).await?;
            }
            RequestSubCommand::GetPrices { signal_ids } => {
                let ids = signal_ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                let response = client.get_prices(&ids).await?;
                println!("{:?}", response);
            }
        }

        Ok(())
    }
}
