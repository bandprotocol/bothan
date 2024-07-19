use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
}
