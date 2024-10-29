use semver::Version;
use serde::ser::SerializeStruct;
use serde::Serialize;
use tokio::time::Duration;

pub const HEARTBEAT_INTERVAL: Duration = Duration::new(60, 0);

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    Records,
    Heartbeat,
}

pub struct BothanInfo {
    pub supported_sources: Vec<String>,
    pub version: Version,
    pub registry_hash: String,
}

impl BothanInfo {
    pub fn new(
        supported_sources: Vec<String>,
        version: Version,
        registry_hash: String,
    ) -> BothanInfo {
        BothanInfo {
            supported_sources,
            version,
            registry_hash,
        }
    }
}

impl Serialize for BothanInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BothanInfo", 4)?;
        state.serialize_field("supported_sources", &self.supported_sources)?;
        state.serialize_field("version", &self.version.to_string())?;
        state.serialize_field("registry_hash", &self.registry_hash)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct Entry<T: Serialize> {
    pub uuid: String,
    pub topic: Topic,
    pub data: T,
}

impl<T: Serialize> Entry<T> {
    pub fn new(uuid: String, topic: Topic, data: T) -> Entry<T> {
        Entry { uuid, topic, data }
    }
}
