#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("invalid provider: {0}")]
    RpcTransportError(#[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>),
}
