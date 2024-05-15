use serde::{Deserialize, Serialize};

use crate::api::types::channels::Channel;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    Subscribe,
    Unsubscribe,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub type_: RequestType,
    pub product_ids: Vec<String>,
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionChannel {
    pub name: String,
    pub product_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscriptions {
    pub channels: Vec<SubscriptionChannel>,
    pub product_ids: Option<Vec<String>>,
    pub account_ids: Option<Vec<String>>,
}
