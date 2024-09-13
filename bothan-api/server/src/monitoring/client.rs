use rand::rngs::OsRng;
use rand::RngCore;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::Serialize;

use bothan_core::store::ActiveSignalIDs;

use crate::monitoring::error::PostError;
use crate::monitoring::signer::Signer;
use crate::monitoring::types::{BothanInfo, Entry, Topic};
use crate::proto::price::Price;
use crate::VERSION;

pub struct Client {
    url: String,
    client: reqwest::Client,
    signer: Signer,
}

impl Client {
    pub fn new<T: Into<String>>(url: T, signer: Signer) -> Self {
        Self {
            url: url.into(),
            client: reqwest::Client::new(),
            signer,
        }
    }

    pub async fn post_price(&self, prices: Vec<Price>) -> Result<Response, PostError> {
        self.post(Topic::Price, prices).await
    }

    pub async fn post_heartbeat(
        &self,
        active_signal_ids: ActiveSignalIDs,
    ) -> Result<Response, PostError> {
        let bothan_info = BothanInfo::new(
            VERSION.to_string(),
            active_signal_ids.into_iter().collect::<Vec<String>>(),
        );

        self.post(Topic::Heartbeat, bothan_info).await
    }

    async fn post<T: Serialize>(&self, topic: Topic, data: T) -> Result<Response, PostError> {
        let mut uuid_bytes = [0u8; 16];
        OsRng.fill_bytes(&mut uuid_bytes);
        let uuid = hex::encode(uuid_bytes);

        let entry = Entry::new(uuid, topic, data);

        let mut header = HeaderMap::new();
        let public_key = self.signer.public_key();
        header.insert("X-Public-Key", public_key.parse()?);
        let signature = self.signer.sign(serde_json::to_string(&entry)?.as_bytes());
        header.insert("X-Signature", hex::encode(&signature).parse()?);

        let resp = self
            .client
            .post(self.url.clone())
            .headers(header)
            .json(&entry)
            .send()
            .await?;
        Ok(resp)
    }
}
