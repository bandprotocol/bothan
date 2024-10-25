use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::ClientBuilder;

use crate::ipfs::client::IpfsClient;

pub struct IpfsClientBuilder {
    url: String,
    header: HeaderMap,
    timeout: Option<Duration>,
}

impl IpfsClientBuilder {
    pub fn new(url: String) -> Self {
        IpfsClientBuilder {
            url,
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
