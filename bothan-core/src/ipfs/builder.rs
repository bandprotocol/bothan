//! Bothan core IPFS client builder module.
//!
//! Provides builder for constructing IPFS clients.

use std::time::Duration;

use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::ipfs::client::IpfsClient;

pub struct IpfsClientBuilder {
    url: String,
    header: HeaderMap,
    timeout: Option<Duration>,
}

impl IpfsClientBuilder {
    pub fn new<T: Into<String>>(url: T) -> Self {
        IpfsClientBuilder {
            url: url.into(),
            header: HeaderMap::new(),
            timeout: None,
        }
    }

    pub fn with_header<K: Into<HeaderName>>(mut self, key: K, value: HeaderValue) -> Self {
        self.header.insert(key.into(), value);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<IpfsClient, reqwest::Error> {
        let mut client_builder = ClientBuilder::new().default_headers(self.header);
        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }
        let client = client_builder.build()?;

        Ok(IpfsClient::new(self.url, client))
    }
}
