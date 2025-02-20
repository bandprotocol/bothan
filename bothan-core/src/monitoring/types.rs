use tokio::time::Duration;

pub use entry::Entry;
pub use info::BothanInfo;
pub use record::{
    OperationRecord, ProcessRecord, SignalComputationRecord, SignalTransactionRecord, SourceRecord,
};
pub use topic::Topic;

mod entry;
mod info;
mod record;
mod topic;

pub const HEARTBEAT_INTERVAL: Duration = Duration::new(60, 0);
