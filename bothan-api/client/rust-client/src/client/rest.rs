use reqwest::{Client, Error, Url};
use std::string::ParseError;

use crate::proto::bothan::v1::{GetInfoResponse, GetPricesResponse};

pub struct RestClient {
    url: Url,
    client: Client,
}

impl RestClient {
    pub fn new(url: String) -> Result<Self, ParseError> {
        Ok(RestClient {
            url: Url::parse(&url)?,
            client: Client::new(),
        })
    }

    async fn get_info(&self) -> Result<GetInfoResponse, Error> {
        let mut url = self.url.clone();
        url.set_path("/info");
        let response = self.client.get(url).send().await?.error_for_status()?;
        let get_info_response = response.json().await?;
        Ok(get_info_response)
    }

    async fn update_registry(&self, ipfs_hash: &str, version: &str) -> Result<(), Error> {
        let mut url = self.url.clone();
        url.set_path("/registry");
        let payload = serde_json::json!({ "ipfs_hash": ipfs_hash, "version": version });
        let _ = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn push_monitoring_records(&self, uuid: &str, tx_hash: &str) -> Result<(), Error> {
        let mut url = self.url.clone();
        url.set_path("/monitoring_records");
        let payload = serde_json::json!({ "uuid": uuid, "tx_hash": tx_hash });
        let _ = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    async fn get_prices(&self, signal_ids: &[&str]) -> Result<GetPricesResponse, Error> {
        let mut url = self.url.clone();
        url.set_path("/prices");
        let payload = serde_json::json!({ "signal_ids": signal_ids });
        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        let get_prices_response = response.json().await?;
        Ok(get_prices_response)
    }
}
