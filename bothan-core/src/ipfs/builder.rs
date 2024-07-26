use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::ClientBuilder;

use crate::ipfs::client::IpfsClient;

pub struct IpfsClientBuilder {
    url: String,
    header: HeaderMap,
}

impl IpfsClientBuilder {
    pub fn new(url: String) -> Self {
        IpfsClientBuilder {
            url,
            header: HeaderMap::new(),
        }
    }

    pub fn with_header<K: Into<HeaderName>>(mut self, key: K, value: HeaderValue) -> Self {
        self.header.insert(key.into(), value);
        self
    }

    pub fn build(self) -> Result<IpfsClient, reqwest::Error> {
        let client = ClientBuilder::new().default_headers(self.header).build()?;
        Ok(IpfsClient::new(self.url, client))
    }
}
