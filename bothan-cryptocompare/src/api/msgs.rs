use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "TYPE")]
pub enum Packet {
    #[serde(rename = "4000")]
    SessionWelcome(Message),

    #[serde(rename = "4001")]
    StreamerError(Message),

    #[serde(rename = "4002")]
    RateLimitError(Message),

    #[serde(rename = "4003")]
    SubscriptionError(Message),

    #[serde(rename = "4004")]
    SubscriptionValidationError(Message),

    #[serde(rename = "4005")]
    SubscriptionAccepted(Message),

    #[serde(rename = "4006")]
    SubscriptionRejected(Message),

    #[serde(rename = "4007")]
    SubscriptionAddComplete(Message),

    #[serde(rename = "4008")]
    SubscriptionRemoveComplete(Message),

    #[serde(rename = "4009")]
    SubscriptionRemoveAllComplete(Message),

    #[serde(rename = "4010")]
    SubscriptionWarning(Message),

    #[serde(rename = "4011")]
    AuthenticationWarning(Message),

    #[serde(rename = "4012")]
    MessageValidationError(Message),

    #[serde(rename = "4013")]
    Heartbeat(Message),

    #[serde(rename = "1101")]
    RefTickAdaptive(ReferenceTick),

    #[serde(rename = "985")]
    RefTickAdaptiveWithConversion(ReferenceTick),

    #[serde(rename = "987")]
    RefTickAdaptiveWithInversion(ReferenceTick),

    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReferenceTick {
    pub instrument: String,
    pub value: f64,
    #[serde(rename = "VALUE_LAST_UPDATE_TS")]
    pub last_update: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Message {
    message: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
