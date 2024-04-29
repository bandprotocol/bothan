use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Ping,
    Subscribe,
    Unsubscribe,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMessage<T> {
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMessageResponse {
    pub error: Option<String>,
    pub method: String,
    pub req_id: Option<usize>,
    pub success: bool,
    pub time_in: String,
    pub time_out: String,
}
