use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Op {
    Subscribe,
    Unsubscribe,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    Spot,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebSocketMessage<T> {
    pub op: Op,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<T>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessageResponse {
    pub event: String,
    pub arg: Option<PriceRequestArgument>,
    pub code: Option<String>,
    pub msg: Option<String>,
    pub conn_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceRequestArgument {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}
