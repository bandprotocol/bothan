use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request<T> {
    /// The operation type (subscribe or unsubscribe).
    pub op: Operation,
    /// The arguments for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<T>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Subscribe,
    Unsubscribe,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    /// The event name.
    pub event: String,
    /// The argument related to the response.
    pub arg: Option<T>,
    /// The response code, if any.
    pub code: Option<String>,
    /// The response message, if any.
    pub msg: Option<String>,
    /// The connection ID.
    pub conn_id: String,
}
