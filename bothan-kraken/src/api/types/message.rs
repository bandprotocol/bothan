use serde::{Deserialize, Serialize};

/// Represents the method type for a public message.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Ping,
    Subscribe,
    Unsubscribe,
}

/// Represents a public message with optional parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMessage<T> {
    /// The method of the message.
    pub method: Method,
    /// The parameters of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
    /// The request ID of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<usize>,
}

/// Represents the response to a public message.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicMessageResponse {
    /// The error message, if any.
    pub error: Option<String>,
    /// The method of the message.
    pub method: String,
    /// The request ID of the message.
    pub req_id: Option<usize>,
    /// Whether the request was successful.
    pub success: bool,
    /// The time the request was received.
    pub time_in: String,
    /// The time the response was sent.
    pub time_out: String,
}
