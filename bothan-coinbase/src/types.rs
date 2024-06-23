use std::time::Duration;

/// The default size for channels.
pub const DEFAULT_CHANNEL_SIZE: usize = 100;

/// The default timeout duration.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(720);

/// Commands that can be sent to the Coinbase service.
pub(crate) enum Command {
    /// Subscribe to a list of product IDs.
    Subscribe(Vec<String>),
}
