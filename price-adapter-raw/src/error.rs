use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("service already started")]
    AlreadyStarted,

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("cannot query price data; please check error log.")]
    GeneralQueryPriceError(),

    #[error("tokio-tungstenite error: {0}")]
    TokioTungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("({0}) cannot parse object; {1}")]
    ParsingError(String, String),

    #[error("response status is not OK; got {0}")]
    ResponseStatusNotOk(StatusCode),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("cannot use {0} as quote")]
    ZeroPrice(String),
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParsingError("std::num::ParseFloatError".into(), err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParsingError("std::num::ParseIntError".into(), err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::ParsingError("serde_json::Error".into(), err.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Self::ParsingError("chrono::ParseError".into(), err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::ParsingError(
            "reqwest::header::InvalidHeaderValue".into(),
            err.to_string(),
        )
    }
}
