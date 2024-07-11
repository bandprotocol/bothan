// @generated
/// QueryPricesRequest is the request type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoSpotPriceRequest {
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="2")]
    pub precision: u32,
}
/// QueryPricesResponse is the response type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoSpotPriceResponse {
    #[prost(message, repeated, tag="1")]
    pub spot_prices: ::prost::alloc::vec::Vec<SpotPriceInfo>,
}
/// AssetPrice contains the price of a signal ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotPriceInfo {
    /// The asset's signal ID.
    #[prost(string, tag="1")]
    pub signal_id: ::prost::alloc::string::String,
    /// The asset's price.
    #[prost(int64, tag="2")]
    pub price: i64,
    /// The asset's price status.
    #[prost(enumeration="SpotPriceInfoStatus", tag="3")]
    pub status: i32,
}
/// Status is the status that defines the AssetPrice result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpotPriceInfoStatus {
    Unspecified = 0,
    Unsupported = 1,
    Unavailable = 2,
    Available = 3,
}
impl SpotPriceInfoStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpotPriceInfoStatus::Unspecified => "UNSPECIFIED",
            SpotPriceInfoStatus::Unsupported => "UNSUPPORTED",
            SpotPriceInfoStatus::Unavailable => "UNAVAILABLE",
            SpotPriceInfoStatus::Available => "AVAILABLE",
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