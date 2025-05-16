use serde::{Deserialize, Serialize};

/// Represents push data from a channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushData<T> {
    /// The argument for the channel.
    pub arg: ChannelArgument,
    /// The data received from the channel.
    pub data: T,
}

/// Represents the argument for a channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArgument {
    /// The name of the channel.
    pub channel: String,
    /// The instrument ID.
    pub inst_id: String,
}
