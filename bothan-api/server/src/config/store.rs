use serde::Deserialize;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct StoreConfig {
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    ".bothan".to_string()
}
