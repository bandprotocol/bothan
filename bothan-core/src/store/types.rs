use std::fmt::Display;

pub(crate) enum Key {
    AssetStore,
    QueryIds,
    ActiveSignalIDs,
    Registry,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Key {
    pub fn as_str(&self) -> &str {
        match self {
            Key::AssetStore => "asset_store",
            Key::QueryIds => "query_ids",
            Key::ActiveSignalIDs => "active_signal_ids",
            Key::Registry => "registry",
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
