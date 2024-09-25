use serde::{Deserialize, Serialize};

/// Represents the operation type for WebSocket messages.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Op {
    Subscribe,
    Unsubscribe,
}

/// Represents the instrument type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    Spot,
}

/// Represents a WebSocket message with generic arguments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebSocketMessage<T> {
    /// The operation type (subscribe or unsubscribe).
    pub op: Op,
    /// The arguments for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<T>>,
}

/// Represents a response to a WebSocket message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessageResponse {
    /// The event name.
    pub event: String,
    /// The argument related to the response.
    pub arg: Option<PriceRequestArgument>,
    /// The response code, if any.
    pub code: Option<String>,
    /// The response message, if any.
    pub msg: Option<String>,
    /// The connection ID.
    pub conn_id: String,
}

/// Represents the arguments for a price request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceRequestArgument {
    /// The name of the channel.
    pub channel: String,
    /// The type of instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// The instrument family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// The instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}
