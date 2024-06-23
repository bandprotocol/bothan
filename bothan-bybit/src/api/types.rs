use serde::{Deserialize, Serialize};

/// Module containing ticker-related types and functions.
pub mod ticker;

/// The default URL for the Bybit API.
pub(crate) const DEFAULT_URL: &str = "https://api.bybit.com/";

/// A generic response structure for API responses.
///
/// # Type Parameters
///
/// * `T` - The type of the result contained in the response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    /// The return code of the response.
    #[serde(rename = "retCode")]
    pub ret_code: usize,
    /// The return message of the response.
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// The result of the response.
    pub result: T,
    /// The timestamp of the response.
    pub time: usize,
}
