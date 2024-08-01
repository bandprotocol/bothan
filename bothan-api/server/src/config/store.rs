use serde::Deserialize;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct StoreConfig {
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    let home = dirs::home_dir().expect("Failed to get home directory");
    let path = home.join(".bothan");
    let path_str = path.to_str().expect("Failed to convert path to string");

    path_str.to_string()
}
