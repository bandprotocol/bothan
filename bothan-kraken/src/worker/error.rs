use thiserror::Error;

use crate::api;

#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("invalid price: {0}")]
    InvalidPrice(f64),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct BuildError(#[from] api::ConnectionError);
