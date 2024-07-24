use std::fmt::Debug;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RegistrySeedConfig {
    Local { path: String },
    Ipfs { hash: String },
}
