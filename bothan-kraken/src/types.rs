use std::time::Duration;

pub const DEFAULT_CHANNEL_SIZE: usize = 100;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(720);

pub(crate) enum Command {
    Subscribe(Vec<String>),
}
