use semver::Version;
use serde::Serialize;
use serde::ser::SerializeStruct;

pub struct BothanInfo {
    pub active_sources: Vec<String>,
    pub version: Version,
    pub registry_hash: String,
}

impl BothanInfo {
    pub fn new(active_sources: Vec<String>, version: Version, registry_hash: String) -> BothanInfo {
        BothanInfo {
            active_sources,
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
        state.serialize_field("active_sources", &self.active_sources)?;
        state.serialize_field("version", &self.version.to_string())?;
        state.serialize_field("registry_hash", &self.registry_hash)?;
        state.end()
    }
}
