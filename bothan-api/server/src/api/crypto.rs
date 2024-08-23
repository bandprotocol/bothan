use std::sync::Arc;

use rust_decimal::RoundingStrategy;
use semver::Version;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use tracing::{error, info, warn};

use bothan_core::manager::crypto_asset_info::error::SetRegistryError;
use bothan_core::manager::crypto_asset_info::types::PriceState;
use bothan_core::manager::CryptoAssetInfoManager;

use crate::api::utils::{parse_price_state, registry_resp};
use crate::proto::query::query_server::Query;
use crate::proto::query::{
    Price, PriceRequest, PriceResponse, PriceStatus, SetActiveSignalIdRequest,
    SetActiveSignalIdResponse, UpdateRegistryRequest, UpdateRegistryResponse, UpdateStatusCode,
};

pub const PRECISION: u32 = 9;

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<RwLock<CryptoAssetInfoManager<'static>>>,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: CryptoAssetInfoManager<'static>) -> Self {
        CryptoQueryServer {
            manager: Arc::new(RwLock::new(manager)),
        }
    }
}

#[tonic::async_trait]
impl Query for CryptoQueryServer {
    async fn update_registry(
        &self,
        request: Request<UpdateRegistryRequest>,
    ) -> Result<Response<UpdateRegistryResponse>, Status> {
        info!("received update registry request");
        let update_registry_request = request.into_inner();

        let version = Version::parse(&update_registry_request.version)
            .map_err(|_| Status::invalid_argument("Invalid version string"))?;

        let mut manager = self.manager.write().await;
        let set_registry_result = manager
            .set_registry_from_ipfs(&update_registry_request.ipfs_hash, version)
            .await;

        match set_registry_result {
            Ok(_) => Ok(registry_resp(UpdateStatusCode::Ok)),
            Err(SetRegistryError::InvalidRegistry(e)) => {
                error!("Invalid registry: {}", e);
                Ok(registry_resp(UpdateStatusCode::InvalidRegistry))
            }
            Err(SetRegistryError::FailedToRetrieve(_)) => {
                Ok(registry_resp(UpdateStatusCode::FailedToGetRegistry))
            }
            Err(SetRegistryError::UnsupportedVersion) => {
                Ok(registry_resp(UpdateStatusCode::UnsupportedVersion))
            }
            Err(SetRegistryError::FailedToParse) => {
                error!("Failed to parse registry");
                Err(Status::invalid_argument("Registry is invalid"))
            }
            Err(SetRegistryError::InvalidHash) => {
                error!("Invalid IPFS hash");
                Err(Status::invalid_argument("Invalid IPFS hash"))
            }
            Err(SetRegistryError::FailedToSetRegistry(_)) => {
                error!("Failed to set registry");
                Err(Status::internal("Failed to set registry"))
            }
        }
    }

    async fn set_active_signal_id(
        &self,
        request: Request<SetActiveSignalIdRequest>,
    ) -> Result<Response<SetActiveSignalIdResponse>, Status> {
        info!("received set active signal id request");
        let update_registry_request = request.into_inner();
        let mut manager = self.manager.write().await;
        let set_result = manager
            .set_active_signal_ids(update_registry_request.signal_ids)
            .await;

        match set_result {
            Ok(_) => {
                info!("successfully set active signal ids");
                Ok(Response::new(SetActiveSignalIdResponse { success: true }))
            }
            Err(e) => {
                error!("failed to set active signal ids: {}", e);
                Ok(Response::new(SetActiveSignalIdResponse { success: false }))
            }
        }
    }

    async fn get_prices(
        &self,
        request: Request<PriceRequest>,
    ) -> Result<Response<PriceResponse>, Status> {
        info!("received get price request");
        let price_request = request.into_inner();
        let manager = self.manager.read().await;
        let price_states = manager
            .get_prices(price_request.signal_ids.clone())
            .await
            .map_err(|e| {
                error!("failed to get prices: {}", e);
                Status::internal("Failed to get prices")
            })?;

        let prices = price_request
            .signal_ids
            .into_iter()
            .zip(price_states)
            .map(|(id, state)| parse_price_state(id, state))
            .collect();

        Ok(Response::new(PriceResponse { prices }))
    }
}
