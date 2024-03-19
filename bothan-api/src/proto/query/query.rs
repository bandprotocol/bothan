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
    /// PriceOption defines the price option of a symbol.
    #[prost(enumeration="PriceOption", tag="3")]
    pub price_option: i32,
}
/// PriceOption defines the price option of a price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceOption {
    /// PRICE_OPTION_UNSPECIFIED defines an unspecified price option.
    Unspecified = 0,
    /// PRICE_OPTION_UNSUPPORTED defines an unsupported price option.
    Unsupported = 1,
    /// PRICE_OPTION_UNAVAILABLE defines an unavailable price option.
    Unavailable = 2,
    /// PRICE_OPTION_AVAILABLE defines an available price option.
    Available = 3,
}
impl PriceOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceOption::Unspecified => "PRICE_OPTION_UNSPECIFIED",
            PriceOption::Unsupported => "PRICE_OPTION_UNSUPPORTED",
            PriceOption::Unavailable => "PRICE_OPTION_UNAVAILABLE",
            PriceOption::Available => "PRICE_OPTION_AVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_OPTION_UNSUPPORTED" => Some(Self::Unsupported),
            "PRICE_OPTION_UNAVAILABLE" => Some(Self::Unavailable),
            "PRICE_OPTION_AVAILABLE" => Some(Self::Available),
            _ => None,
        }
    }
}
include!("query.tonic.rs");
// @@protoc_insertion_point(module)