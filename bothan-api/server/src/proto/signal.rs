// @generated
/// UpdateRegistryRequest is the request message for the UpdateRegistry RPC method.
/// It contains the IPFS hash and version information needed to update the registry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistryRequest {
    /// The IPFS hash pointing to the registry data.
    #[prost(string, tag="1")]
    pub ipfs_hash: ::prost::alloc::string::String,
    /// The version of the registry.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// SetActiveSignalIdsRequest is the request message for the SetActiveSignalIds RPC method.
/// It contains the list of signal IDs that should be marked as active.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActiveSignalIdsRequest {
    /// A list of signal IDs to be set as active.
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PushMonitoringRecordsRequest is the request message for the PushMonitoringRecords RPC method.
/// It contains the uuid of the records that should be pushed to the monitoring service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMonitoringRecordsRequest {
    /// A list of monitoring records to be pushed to the monitoring service.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
}
include!("signal.tonic.rs");
// @@protoc_insertion_point(module)