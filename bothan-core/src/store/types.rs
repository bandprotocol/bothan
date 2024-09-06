use std::fmt::Display;

pub(crate) enum Key<'a> {
    AssetStore { source_id: &'a str, id: &'a str },
    QueryIDs { source_id: &'a str },
    ActiveSignalIDs,
    Registry,
}

impl<'a> Display for Key<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Key::AssetStore { source_id, id } => format!("asset_store::{}::{}", source_id, id),
            Key::QueryIDs { source_id } => format!("query_id::{}", source_id),
            Key::ActiveSignalIDs => "active_signal_ids".to_string(),
            Key::Registry => "registry".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl<'a> Key<'a> {
    pub fn to_prefixed_bytes(&self) -> Vec<u8> {
        let prefix = "bothan::".as_bytes();
        let content = self.to_string().into_bytes();
        [prefix, &content].concat()
    }
}
