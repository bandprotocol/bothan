use std::sync::Arc;

use crate::manager::CryptoAssetInfoManager;
use crate::proto::query::query_server::Query;
use crate::proto::query::{
    PriceRequest, PriceResponse, SetActiveSignalIdRequest, SetActiveSignalIdResponse,
    UpdateRegistryRequest, UpdateRegistryResponse,
};
use crate::utils::arc_mutex;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::error;

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<Mutex<CryptoAssetInfoManager>>,
    // TODO: For future use
    _ipfs_base_url: String,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: CryptoAssetInfoManager, ipfs_base_url: String) -> Self {
        CryptoQueryServer {
            manager: arc_mutex!(manager),
            _ipfs_base_url: ipfs_base_url,
        }
    }
}

#[tonic::async_trait]
impl Query for CryptoQueryServer {
    async fn update_registry(
        &self,
        _: Request<UpdateRegistryRequest>,
    ) -> Result<Response<UpdateRegistryResponse>, Status> {
        todo!()
    }

    async fn set_active_signal_id(
        &self,
        request: Request<SetActiveSignalIdRequest>,
    ) -> Result<Response<SetActiveSignalIdResponse>, Status> {
        let update_registry_request = request.into_inner();
        let mut manager = self.manager.lock().await;
        manager
            .set_active_signal_ids(update_registry_request.signal_ids)
            .await;
        Ok(Response::new(SetActiveSignalIdResponse { success: true }))
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
