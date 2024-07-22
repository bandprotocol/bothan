use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::manager::CryptoAssetInfoManager;
use crate::proto::query::query_server::Query;
use crate::proto::query::{
    PriceRequest, PriceResponse, SetActiveSignalIdRequest, SetActiveSignalIdResponse,
    UpdateRegistryRequest, UpdateRegistryResponse,
};
use crate::utils::arc_mutex;

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<Mutex<CryptoAssetInfoManager>>,
    // TODO: placeholder for future implementation
    ipfs_client: Option<()>,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: CryptoAssetInfoManager, ipfs_client: Option<()>) -> Self {
        CryptoQueryServer {
            manager: arc_mutex!(manager),
            ipfs_client,
        }
    }
}

#[tonic::async_trait]
impl Query for CryptoQueryServer {
    async fn update_registry(
        &self,
        _: Request<UpdateRegistryRequest>,
    ) -> Result<Response<UpdateRegistryResponse>, Status> {
        // TODO: Implement
        match self.ipfs_client {
            Some(_) => Err(Status::unimplemented("IPFS client is not implemented")),
            None => Err(Status::not_found("registry update is disabled")),
        }
    }

    async fn set_active_signal_id(
        &self,
        request: Request<SetActiveSignalIdRequest>,
    ) -> Result<Response<SetActiveSignalIdResponse>, Status> {
        let update_registry_request = request.into_inner();
        let mut manager = self.manager.lock().await;

        let set_result = manager
            .set_active_signal_ids(update_registry_request.signal_ids)
            .await;

        match set_result {
            Ok(_) => Ok(Response::new(SetActiveSignalIdResponse { success: true })),
            Err(_) => Ok(Response::new(SetActiveSignalIdResponse { success: false })),
        }
    }

    async fn get_price(
        &self,
        request: Request<PriceRequest>,
    ) -> Result<Response<PriceResponse>, Status> {
        let price_request = request.into_inner();
        let mut manager = self.manager.lock().await;
        match manager.get_prices(price_request.signal_ids).await {
            Ok(prices) => Ok(Response::new(PriceResponse { prices })),
            Err(e) => {
                error!("Failed to get prices: {:?}", e);
                Err(Status::internal("Failed to get prices"))
            }
        }
    }
}
