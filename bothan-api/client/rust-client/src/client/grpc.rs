//! Bothan API Rust gRPC client implementation.
//!
//! Provides async gRPC client for Bothan API Server.

use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

use crate::proto::bothan::v1::{
    BothanServiceClient, GetInfoRequest, GetInfoResponse, GetPricesRequest, GetPricesResponse,
    PushMonitoringRecordsRequest, UpdateRegistryRequest,
};

pub struct GrpcClient {
    client: Arc<Mutex<BothanServiceClient<Channel>>>,
}

/// Async gRPC client for Bothan API Server.
impl GrpcClient {
    pub fn new(client: BothanServiceClient<Channel>) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn connect(addr: &str) -> Result<Self, tonic::transport::Error> {
        let endpoint = Endpoint::from_str(addr)?;
        let client = Arc::new(Mutex::new(BothanServiceClient::connect(endpoint).await?));
        Ok(GrpcClient { client })
    }

    pub async fn get_info(&self) -> Result<GetInfoResponse, Status> {
        let get_info_request = GetInfoRequest {};
        let request = Request::new(get_info_request);
        let response = self.client.lock().await.get_info(request).await?;
        Ok(response.into_inner())
    }

    pub async fn update_registry(&self, ipfs_hash: &str, version: &str) -> Result<(), Status> {
        let update_registry_request = UpdateRegistryRequest {
            ipfs_hash: ipfs_hash.into(),
            version: version.into(),
        };
        let request = Request::new(update_registry_request);
        let _ = self.client.lock().await.update_registry(request).await?;
        Ok(())
    }

    pub async fn push_monitoring_records(
        &self,
        uuid: &str,
        tx_hash: &str,
        signal_ids: &[&str],
    ) -> Result<(), Status> {
        let push_monitoring_records_request = PushMonitoringRecordsRequest {
            uuid: uuid.into(),
            tx_hash: tx_hash.into(),
            signal_ids: signal_ids.iter().map(|s| s.to_string()).collect(),
        };
        let request = Request::new(push_monitoring_records_request);
        let _ = self
            .client
            .lock()
            .await
            .push_monitoring_records(request)
            .await?;
        Ok(())
    }

    pub async fn get_prices(&self, signal_ids: &[&str]) -> Result<GetPricesResponse, Status> {
        let get_prices_request = GetPricesRequest {
            signal_ids: signal_ids.iter().map(|s| s.to_string()).collect(),
        };
        let request = Request::new(get_prices_request);
        let response = self.client.lock().await.get_prices(request).await?;
        Ok(response.into_inner())
    }
}
