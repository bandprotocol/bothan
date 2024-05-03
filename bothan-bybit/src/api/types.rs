use serde::{Deserialize, Serialize};

pub mod ticker;

pub(crate) const DEFAULT_URL: &str = "https://api.bybit.com/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "retCode")]
    pub ret_code: usize,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: T,
    pub time: usize,
}
