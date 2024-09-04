// @generated
/// GetPricesRequest is the request message for the GetPrices RPC method.
/// It contains the list of signal IDs for which prices are requested.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPricesRequest {
    /// A list of signal IDs for which the prices are being requested.
    #[prost(string, repeated, tag="1")]
    pub signal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetPricesResponse is the response message for the GetPrices RPC method.
/// It contains a list of prices corresponding to the requested signal IDs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPricesResponse {
    /// A list of prices for the requested signal IDs.
    #[prost(message, repeated, tag="1")]
    pub prices: ::prost::alloc::vec::Vec<Price>,
}
/// Price contains the price information for a signal ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The signal ID associated with this price.
    #[prost(string, tag="1")]
    pub signal_id: ::prost::alloc::string::String,
    /// The price of the asset associated with this signal ID.
    #[prost(int64, tag="2")]
    pub price: i64,
    /// The status of the price (e.g., available, unavailable).
    #[prost(enumeration="Status", tag="3")]
    pub status: i32,
}
/// PriceStatus is an enum that defines the status of the price for a signal ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Default status, should not be used.
    Unspecified = 0,
    /// Indicates that the price for the signal ID is not supported.
    Unsupported = 1,
    /// Indicates that the price for the signal ID is currently unavailable.
    Unavailable = 2,
    /// Indicates that the price for the signal ID is available.
    Available = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unspecified => "UNSPECIFIED",
            Status::Unsupported => "UNSUPPORTED",
            Status::Unavailable => "UNAVAILABLE",
            Status::Available => "AVAILABLE",
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
include!("price.tonic.rs");
// @@protoc_insertion_point(module)