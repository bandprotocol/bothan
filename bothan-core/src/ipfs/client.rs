//! Bothan core IPFS client implementation.
//!
//! Provides async HTTP client for IPFS operations.

use reqwest::{Client, StatusCode};

use crate::ipfs::error::Error;

pub struct IpfsClient {
    url: String,
    client: Client,
}

impl IpfsClient {
    pub fn new(url: String, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_ipfs<T: AsRef<str>>(&self, hash: T) -> Result<String, Error> {
        let url = format!("{}/ipfs/{}", self.url, hash.as_ref());
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::RequestFailed(e.to_string()))?;

        match resp.status() {
            StatusCode::OK => resp
                .text()
                .await
                .map_err(|e| Error::RequestFailed(e.to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => Err(Error::DoesNotExist),
            _ => Err(Error::NonZeroStatus(resp.status().as_u16())),
        }
    }
}
