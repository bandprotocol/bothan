use std::sync::Arc;

use semver::Version;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::manager::crypto_asset_info::error::SetRegistryError;
use crate::manager::CryptoAssetInfoManager;
use crate::proto::query::query_server::Query;
use crate::proto::query::{
    PriceRequest, PriceResponse, SetActiveSignalIdRequest, SetActiveSignalIdResponse,
    UpdateRegistryRequest, UpdateRegistryResponse, UpdateStatusCode,
};
use crate::utils::arc_mutex;

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<Mutex<CryptoAssetInfoManager>>,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: CryptoAssetInfoManager) -> Self {
        CryptoQueryServer {
            manager: arc_mutex!(manager),
        }
    }
}

#[tonic::async_trait]
impl Query for CryptoQueryServer {
    async fn update_registry(
        &self,
        request: Request<UpdateRegistryRequest>,
    ) -> Result<Response<UpdateRegistryResponse>, Status> {
        let update_registry_request = request.into_inner();

        let version = Version::parse(&update_registry_request.version).map_err(|e| {
            error!("Failed to parse version: {:?}", e);
            Status::invalid_argument("Version given is not properly formatted")
        })?;

        let mut manager = self.manager.lock().await;
        manager
            .set_registry(&update_registry_request.ipfs_hash, version)
            .await
            .map(|_| {
                Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::Ok.into(),
                })
            })
            .or_else(|e| match e {
                SetRegistryError::UnsupportedVersion => Ok(Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::UnsupportedVersion.into(),
                })),
                SetRegistryError::InvalidRegistry => Ok(Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::InvalidRegistry.into(),
                })),
                SetRegistryError::FailedToRetrieve => Ok(Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::FailedToGetRegistry.into(),
                })),
            })
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
