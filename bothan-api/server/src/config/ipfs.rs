use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct IpfsConfig {
    pub url: String,
}
