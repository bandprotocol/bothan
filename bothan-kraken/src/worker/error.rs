use thiserror::Error;

use crate::api;

#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("value is not a valid decimal")]
    NaN,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct BuildError(#[from] api::ConnectionError);
