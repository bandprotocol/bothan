#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    AlloyAddressError(#[from] alloy_primitives::AddressError),

    #[error("{0}")]
    AlloyContract(#[from] alloy::contract::Error),

    #[error("{0}")]
    AlloyTransport(#[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>),
}
