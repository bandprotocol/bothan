use std::sync::Arc;

use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::manager::crypto_asset_info::error::{PushMonitoringRecordError, SetRegistryError};
use bothan_lib::store::Store;
use semver::Version;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

use crate::api::utils::parse_price_state;
use crate::proto::bothan::v1::{
    BothanService, GetInfoRequest, GetInfoResponse, GetPricesRequest, GetPricesResponse, Price,
    PushMonitoringRecordsRequest, PushMonitoringRecordsResponse, UpdateRegistryRequest,
    UpdateRegistryResponse,
};

pub const PRECISION: u32 = 9;

/// The `BothanServer` struct represents a server that implements the `BothanService` trait.
pub struct BothanServer<S: Store + 'static> {
    manager: Arc<CryptoAssetInfoManager<S>>,
}

impl<S: Store> BothanServer<S> {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: Arc<CryptoAssetInfoManager<S>>) -> Self {
        BothanServer { manager }
    }
}

// TODO: cleanup logging with span
#[tonic::async_trait]
impl<S: Store> BothanService for BothanServer<S> {
    async fn get_info(
        &self,
        _: Request<GetInfoRequest>,
    ) -> Result<Response<GetInfoResponse>, Status> {
        info!("received get info request");
        let info = self
            .manager
            .get_info()
            .await
            .map_err(|_| Status::internal("Failed to get info"))?;

        let response = Response::new(GetInfoResponse {
            bothan_version: info.bothan_version,
            registry_ipfs_hash: info.registry_hash,
            registry_version_requirement: info.registry_version_requirement,
            active_sources: info.active_sources,
            monitoring_enabled: info.monitoring_enabled,
        });
        debug!("response: {:?}", response);
        Ok(response)
    }

    async fn update_registry(
        &self,
        request: Request<UpdateRegistryRequest>,
    ) -> Result<Response<UpdateRegistryResponse>, Status> {
        info!("received update registry request");
        debug!("request: {:?}", request);
        let update_registry_request = request.into_inner();

        let version = Version::parse(&update_registry_request.version)
            .map_err(|_| Status::invalid_argument("Invalid version string"))?;

        let set_registry_result = self
            .manager
            .set_registry_from_ipfs(&update_registry_request.ipfs_hash, version)
            .await;

        match set_registry_result {
            Ok(_) => {
                info!("successfully set registry");
                Ok(Response::new(UpdateRegistryResponse {}))
            }
            Err(SetRegistryError::FailedToRetrieve(e)) => {
                error!("failed to retrieve registry: {}", e);
                Err(Status::not_found("Failed to retrieve registry"))
            }
            Err(SetRegistryError::InvalidRegistry(e)) => {
                error!("invalid registry: {}", e);
                Err(Status::invalid_argument("Registry is invalid"))
            }
            Err(SetRegistryError::UnsupportedVersion) => {
                error!("unsupported registry version");
                Err(Status::invalid_argument("Registry version is unsupported"))
            }
            Err(SetRegistryError::FailedToParse) => {
                error!("failed to parse registry");
                Err(Status::invalid_argument("Unable to parse registry version"))
            }
            Err(SetRegistryError::InvalidHash) => {
                error!("invalid IPFS hash");
                Err(Status::invalid_argument("Invalid IPFS hash"))
            }
            Err(SetRegistryError::FailedToSetRegistry) => {
                error!("failed to set registry");
                Err(Status::internal("Failed to set registry"))
            }
        }
    }

    async fn push_monitoring_records(
        &self,
        request: Request<PushMonitoringRecordsRequest>,
    ) -> Result<Response<PushMonitoringRecordsResponse>, Status> {
        info!("received push monitoring records request");
        debug!("request: {:?}", request);
        let request = request.into_inner();
        let push_result = self
            .manager
            .push_monitoring_record(request.uuid, request.tx_hash)
            .await;

        match push_result {
            Ok(_) => {
                info!("successfully pushed monitoring records");
                Ok(Response::new(PushMonitoringRecordsResponse {}))
            }
            Err(PushMonitoringRecordError::MonitoringNotEnabled) => {
                info!("monitoring not enabled");
                Err(Status::unimplemented("Monitoring not enabled"))
            }
            Err(PushMonitoringRecordError::RecordNotFound) => {
                info!("record not found");
                Err(Status::failed_precondition("Record not found"))
            }
            Err(PushMonitoringRecordError::FailedRequest(e)) => {
                error!("failed to send request to monitoring: {}", e);
                Err(Status::internal(
                    "Failed to send request to monitoring record",
                ))
            }
            Err(PushMonitoringRecordError::FailedToSendPayload(e)) => {
                error!("failed to send payload to monitoring: {}", e);
                Err(Status::internal(
                    "Failed to send payload to monitoring record",
                ))
            }
        }
    }

    async fn get_prices(
        &self,
        request: Request<GetPricesRequest>,
    ) -> Result<Response<GetPricesResponse>, Status> {
        info!("received get price request");
        debug!("request: {:?}", request);
        let price_request = request.into_inner();
        let (uuid, price_states) = self
            .manager
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
            .collect::<Vec<Price>>();
        let response = Response::new(GetPricesResponse { uuid, prices });
        debug!("response: {:?}", response);
        Ok(response)
    }
}
