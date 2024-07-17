// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistryRequest {
    #[prost(string, tag="1")]
    pub ipfs_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistryResponse {
    #[prost(enumeration="UpdateStatusCode", tag="1")]
    pub code: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActiveSignalIdRequest {
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActiveSignalIdResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
/// QueryPricesRequest is the request type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRequest {
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryPricesResponse is the response type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceResponse {
    #[prost(message, repeated, tag="1")]
    pub prices: ::prost::alloc::vec::Vec<Price>,
}
/// AssetPrice contains the price of a signal ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The asset's signal ID.
    #[prost(string, tag="1")]
    pub signal_id: ::prost::alloc::string::String,
    /// The asset's price.
    #[prost(int64, tag="2")]
    pub price: i64,
    /// The asset's price status.
    #[prost(enumeration="PriceStatus", tag="3")]
    pub status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateStatusCode {
    Ok = 0,
    UnsupportedVersion = 1,
    InvalidRegistry = 2,
    FailedToGetRegistry = 3,
}
impl UpdateStatusCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateStatusCode::Ok => "OK",
            UpdateStatusCode::UnsupportedVersion => "UNSUPPORTED_VERSION",
            UpdateStatusCode::InvalidRegistry => "INVALID_REGISTRY",
            UpdateStatusCode::FailedToGetRegistry => "FAILED_TO_GET_REGISTRY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "UNSUPPORTED_VERSION" => Some(Self::UnsupportedVersion),
            "INVALID_REGISTRY" => Some(Self::InvalidRegistry),
            "FAILED_TO_GET_REGISTRY" => Some(Self::FailedToGetRegistry),
            _ => None,
        }
    }
}
/// Status is the status that defines the AssetPrice result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceStatus {
    Unspecified = 0,
    Unsupported = 1,
    Unavailable = 2,
    Available = 3,
}
impl PriceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceStatus::Unspecified => "UNSPECIFIED",
            PriceStatus::Unsupported => "UNSUPPORTED",
            PriceStatus::Unavailable => "UNAVAILABLE",
            PriceStatus::Available => "AVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "UNSUPPORTED" => Some(Self::Unsupported),
            "UNAVAILABLE" => Some(Self::Unavailable),
            "AVAILABLE" => Some(Self::Available),
            _ => None,
        }
    }
}
include!("query.tonic.rs");
// @@protoc_insertion_point(module)