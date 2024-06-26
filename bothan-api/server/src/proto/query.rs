// @generated
/// QueryPricesRequest is the request type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPricesRequest {
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryPricesResponse is the response type for the PriceService/GetPrices RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPricesResponse {
    #[prost(message, repeated, tag="1")]
    pub prices: ::prost::alloc::vec::Vec<PriceData>,
}
/// PriceData defines the data of a symbol price.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceData {
    /// The symbol of the price.
    #[prost(string, tag="1")]
    pub signal_id: ::prost::alloc::string::String,
    /// The price of the symbol.
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    /// PriceStatus defines the price status of a symbol.
    #[prost(enumeration="PriceStatus", tag="3")]
    pub price_status: i32,
}
/// PriceOption defines the price option of a price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceStatus {
    /// PRICE_STATUS_UNSPECIFIED defines an unspecified price status.
    Unspecified = 0,
    /// PRICE_STATUS_UNSUPPORTED defines an unsupported price status.
    Unsupported = 1,
    /// PRICE_STATUS_UNAVAILABLE defines an unavailable price status.
    Unavailable = 2,
    /// PRICE_STATUS_AVAILABLE defines an available price status.
    Available = 3,
}
impl PriceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceStatus::Unspecified => "PRICE_STATUS_UNSPECIFIED",
            PriceStatus::Unsupported => "PRICE_STATUS_UNSUPPORTED",
            PriceStatus::Unavailable => "PRICE_STATUS_UNAVAILABLE",
            PriceStatus::Available => "PRICE_STATUS_AVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_STATUS_UNSUPPORTED" => Some(Self::Unsupported),
            "PRICE_STATUS_UNAVAILABLE" => Some(Self::Unavailable),
            "PRICE_STATUS_AVAILABLE" => Some(Self::Available),
            _ => None,
        }
    }
}
include!("query.tonic.rs");
// @@protoc_insertion_point(module)