// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceDataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceDataResponse {
    #[prost(message, repeated, tag="1")]
    pub price_data_list: ::prost::alloc::vec::Vec<PriceData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
}
include!("price.tonic.rs");
// @@protoc_insertion_point(module)