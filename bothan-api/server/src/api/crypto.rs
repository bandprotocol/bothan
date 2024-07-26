use std::sync::Arc;

use semver::Version;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use bothan_core::manager::crypto_asset_info::error::SetRegistryError;
use bothan_core::manager::crypto_asset_info::types::PriceState;
use bothan_core::manager::CryptoAssetInfoManager;

use crate::proto::query::query_server::Query;
use crate::proto::query::{
    Price, PriceRequest, PriceResponse, PriceStatus, SetActiveSignalIdRequest,
    SetActiveSignalIdResponse, UpdateRegistryRequest, UpdateRegistryResponse, UpdateStatusCode,
};

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<Mutex<CryptoAssetInfoManager>>,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: CryptoAssetInfoManager) -> Self {
        CryptoQueryServer {
            manager: Arc::new(Mutex::new(manager)),
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

        let version = Version::parse(&update_registry_request.version)
            .map_err(|_| Status::invalid_argument("Invalid version string"))?;

        let mut manager = self.manager.lock().await;
        let set_registry_result = manager
            .set_registry_from_ipfs(&update_registry_request.ipfs_hash, version)
            .await;

        match set_registry_result {
            Ok(_) => Ok(Response::new(UpdateRegistryResponse {
                code: UpdateStatusCode::Ok.into(),
            })),
            Err(e) => match e {
                SetRegistryError::InvalidRegistry => Ok(Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::InvalidRegistry.into(),
                })),
                SetRegistryError::FailedToRetrieve(_) => {
                    Ok(Response::new(UpdateRegistryResponse {
                        code: UpdateStatusCode::FailedToGetRegistry.into(),
                    }))
                }
                SetRegistryError::UnsupportedVersion => Ok(Response::new(UpdateRegistryResponse {
                    code: UpdateStatusCode::UnsupportedVersion.into(),
                })),
                SetRegistryError::FailedToParse => Err(Status::invalid_argument(
                    "Registry is incorrectly formatted",
                )),
                SetRegistryError::InvalidHash => Err(Status::invalid_argument("Invalid IPFS hash")),
            },
        }
    }

    async fn set_active_signal_id(
        &self,
        request: Request<SetActiveSignalIdRequest>,
    ) -> Result<Response<SetActiveSignalIdResponse>, Status> {
        info!("received set active signal id request");
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
        match manager.get_prices(&price_request.signal_ids).await {
            Ok(price_states) => {
                let prices = price_request
                    .signal_ids
                    .into_iter()
                    .zip(price_states)
                    .map(|(id, state)| match state {
                        PriceState::Available(price) => Price {
                            signal_id: id.clone(),
                            status: PriceStatus::Available.into(),
                            price,
                        },
                        PriceState::Unavailable => Price {
                            signal_id: id.clone(),
                            status: PriceStatus::Unavailable.into(),
                            price: 0,
                        },
                        PriceState::Unsupported => Price {
                            signal_id: id.clone(),
                            status: PriceStatus::Unsupported.into(),
                            price: 0,
                        },
                    })
                    .collect();

                Ok(Response::new(PriceResponse { prices }))
            }
            Err(e) => {
                error!("Failed to get prices: {:?}", e);
                Err(Status::internal("Failed to get prices"))
            }
        }
    }
}
