use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;
use std::path::Path;

use reqwest::{Client, StatusCode};
use thiserror::Error;

use crate::config::ipfs::IpfsAuthentication;
use crate::registry::signal::Signal;
use crate::tasks::Tasks;

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;

/// The `Registry` type is a HashMap that maps a signal name to its corresponding `Signal`.
#[derive(Clone, Debug)]
pub struct Registry {
    registry: HashMap<String, Signal>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum IpfsError {
    #[error("Failed to send request")]
    SendError,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Failed to parse into registry")]
    ParseError,
}

impl Registry {
    pub fn new(registry: HashMap<String, Signal>) -> Self {
        Self { registry }
    }

    pub async fn try_from_ipfs<T, U>(
        url: T,
        hash: U,
        auth: &IpfsAuthentication,
    ) -> Result<Self, IpfsError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let req_builder = Client::new().get(format!("{}/ipfs/{}", url.as_ref(), hash.as_ref()));
        let req_builder = match auth {
            IpfsAuthentication::Bearer { token } => req_builder.bearer_auth(token),
            IpfsAuthentication::Basic { username, password } => {
                req_builder.basic_auth(username, Some(password))
            }
            IpfsAuthentication::None => req_builder,
        };

        let resp = req_builder.send().await.map_err(|_| IpfsError::SendError)?;
        match resp.status() {
            StatusCode::OK => {
                let registry = resp
                    .json::<HashMap<String, Signal>>()
                    .await
                    .map_err(|_| IpfsError::ParseError)?;
                Ok(Self { registry })
            }
            StatusCode::UNPROCESSABLE_ENTITY => Err(IpfsError::InvalidHash),
            _ => Err(IpfsError::SendError),
        }
    }

    pub fn try_from_path<T: AsRef<Path>>(path: T) -> anyhow::Result<Registry> {
        let file = std::fs::File::open(path.as_ref())?;
        let reader = std::io::BufReader::new(file);
        let registry = serde_json::from_reader(reader)?;
        Ok(Self { registry })
    }

    pub fn keys(&self) -> Keys<'_, String, Signal> {
        self.registry.keys()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.registry.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Signal> {
        self.registry.get(key)
    }

    pub fn values(&self) -> Values<'_, String, Signal> {
        self.registry.values()
    }

    pub fn is_valid(&self) -> bool {
        Tasks::try_from(self.clone()).is_ok()
    }
}
