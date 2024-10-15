use serde::Serialize;
use tokio::time::Duration;

pub const HEARTBEAT_INTERVAL: Duration = Duration::new(60, 0);

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    Price,
    Heartbeat,
}

#[derive(Serialize)]
pub struct BothanInfo {
    pub version: String,
    pub active_signal_ids: Vec<String>,
    pub supported_sources: Vec<String>,
}

impl BothanInfo {
    pub fn new(
        version: String,
        active_signal_ids: Vec<String>,
        supported_sources: Vec<String>,
    ) -> BothanInfo {
        BothanInfo {
            version,
            active_signal_ids,
            supported_sources,
        }
    }
}

#[derive(Serialize)]
pub struct Entry<T: Serialize> {
    pub uuid: String,
    pub topic: Topic,
    pub data: T,
}

impl<T: Serialize> Entry<T> {
    pub fn new<U: Into<String>>(uuid: U, topic: Topic, data: T) -> Entry<T> {
        Entry {
            uuid: uuid.into(),
            topic,
            data,
        }
    }
}
