use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("cannot query price data; please check error log.")]
    GeneralQueryPriceError(),

    #[error("tokio-tungstenite error: {0}")]
    TokioTungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    ParsingFloatError(#[from] std::num::ParseFloatError),

    #[error("response status is not OK; got {0}")]
    ResponseStatusNotOk(StatusCode),

    #[error("cannot query price for id {0}")]
    NotFound(String),

    #[error("cannot use {0} as quote")]
    ZeroPrice(String),
}
