//! Bothan core monitoring topic type.
//!
//! Defines the `Topic` enum for monitoring message types.

use serde::Serialize;

/// Topic for monitoring messages.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    /// Monitoring record message.
    Record,
    /// Heartbeat message.
    Heartbeat,
}
