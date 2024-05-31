use std::time::Duration;

/// The default size for channels.
pub const DEFAULT_CHANNEL_SIZE: usize = 100;

/// The default timeout duration for WebSocket operations.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(720);

/// Represents the various commands that can be sent to the Kraken service.
pub(crate) enum Command {
    /// Command to subscribe to a list of symbols.
    Subscribe(Vec<String>),
}
