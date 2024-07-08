use thiserror::Error;

use crate::api;

#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("value is not a valid decimal: {0}")]
    Underflow(#[from] rust_decimal::Error),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct BuildError(#[from] api::ConnectionError);
