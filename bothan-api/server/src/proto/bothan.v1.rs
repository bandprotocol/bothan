// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {
}
/// GetInfoResponse defines the response message for the GetInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// The bothan version
    #[prost(string, tag="1")]
    pub bothan_version: ::prost::alloc::string::String,
    /// The IPFS hash pointing to the registry data.
    #[prost(string, tag="2")]
    pub registry_ipfs_hash: ::prost::alloc::string::String,
    /// The version requirements for the registry.
    #[prost(string, tag="3")]
    pub registry_version_requirement: ::prost::alloc::string::String,
}
/// UpdateRegistryRequest defines the request message for the UpdateRegistry RPC method.
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UpdateRegistryResponse {
}
/// PushMonitoringRecordsRequest defines the request message for the PushMonitoringRecords RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMonitoringRecordsRequest {
    /// The uuid of a list of monitoring records to be pushed to the monitoring service.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// The tx hash of the transaction associated with the monitoring records.
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
}
/// PushMonitoringRecordsResponse defines the response message for the PushMonitoringRecords RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PushMonitoringRecordsResponse {
}
/// GetPricesRequest defines the request message for the GetPrices RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPricesRequest {
    /// A list of signal IDs for which the prices are being requested.
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetPricesResponse defines the response message for the GetPrices RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPricesResponse {
    /// A unique identifier for the response.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// A list of prices for the requested signal IDs.
    #[prost(message, repeated, tag="2")]
    pub prices: ::prost::alloc::vec::Vec<Price>,
}
/// Price defines the price information for a signal ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The signal ID.
    #[prost(string, tag="1")]
    pub signal_id: ::prost::alloc::string::String,
    /// The price value associated with this signal ID.
    #[prost(uint64, tag="2")]
    pub price: u64,
    /// The status of the signal ID.
    #[prost(enumeration="Status", tag="3")]
    pub status: i32,
}
/// Status defines the status for a signal ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Default status, should not be used.
    Unspecified = 0,
    /// Indicates that the signal ID is not supported.
    Unsupported = 1,
    /// Indicates that the signal ID is currently unavailable.
    Unavailable = 2,
    /// Indicates that the signal ID is available.
    Available = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unspecified => "STATUS_UNSPECIFIED",
            Status::Unsupported => "STATUS_UNSUPPORTED",
            Status::Unavailable => "STATUS_UNAVAILABLE",
            Status::Available => "STATUS_AVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "STATUS_UNSUPPORTED" => Some(Self::Unsupported),
            "STATUS_UNAVAILABLE" => Some(Self::Unavailable),
            "STATUS_AVAILABLE" => Some(Self::Available),
            _ => None,
        }
    }
}
include!("bothan.v1.tonic.rs");
// @@protoc_insertion_point(module)