use crate::monitoring::error::Error as MonitoringError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SetRegistryError {
    #[error("Failed to set registry")]
    FailedToSetRegistry,

    #[error("Failed to get registry from IPFS")]
    FailedToRetrieve(String),

    #[error("Unsupported version")]
    UnsupportedVersion,

    #[error("Failed to parse into registry")]
    FailedToParse,

    #[error("Registry is invalid: {0}")]
    InvalidRegistry(String),

    #[error("Invalid ipfs hash")]
    InvalidHash,
}

#[derive(Debug, Error)]
pub enum PushMonitoringRecordError {
    #[error("Failed to send payload to monitor: {0}")]
    FailedToSendPayload(#[from] MonitoringError),

    #[error("Response from monitoring is non 2xx: {0}")]
    FailedRequest(#[from] reqwest::Error),

    #[error("Monitoring not enabled")]
    MonitoringNotEnabled,

    #[error("Record not found")]
    RecordNotFound,
}

#[derive(Debug, Error)]
pub enum PostHeartbeatError {
    #[error("Failed to get registry hash")]
    FailedToGetRegistryHash,

    #[error("Failed to send payload to monitor: {0}")]
    FailedToSendPayload(#[from] MonitoringError),

    #[error("Response from monitoring is non 2xx: {0}")]
    FailedRequest(#[from] reqwest::Error),

    #[error("Monitoring not enabled")]
    MonitoringNotEnabled,
}
