//! Bothan core monitoring types.
//!
//! Re-exports types and constants for monitoring records and topics.
//! Contains the `Entry`, `BothanInfo`, and `Topic` types, as well as the `HEARTBEAT_INTERVAL` constant.

pub use entry::Entry;
pub use info::BothanInfo;
pub use record::{
    OperationRecord, ProcessRecord, SignalComputationRecord, SignalTransactionRecord, SourceRecord,
};
use tokio::time::Duration;
pub use topic::Topic;

mod entry;
mod info;
mod record;
mod topic;

pub const HEARTBEAT_INTERVAL: Duration = Duration::new(60, 0);
