use reqwest::header::HeaderMap;
use reqwest::Response;
use semver::Version;
use serde::Serialize;
use std::sync::Arc;

use crate::monitoring::error::Error;
use crate::monitoring::records::SignalComputationRecords;
use crate::monitoring::signer::Signer;
use crate::monitoring::types::{BothanInfo, Entry, Topic};
use crate::store::ActiveSignalIDs;

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

    pub async fn post_arced_signal_record<T, U>(
        &self,
        uuid: String,
        records: Arc<SignalComputationRecords<T, U>>,
    ) -> Result<Response, Error>
    where
        T: Serialize + Sized,
        U: Serialize,
    {
        self.post(uuid, Topic::Records, records).await
    }

    pub async fn post_heartbeat(
        &self,
        uuid: String,
        active_signal_ids: ActiveSignalIDs,
        supported_sources: Vec<String>,
        bothan_version: Version,
        registry_hash: String,
    ) -> Result<Response, Error> {
        let bothan_info = BothanInfo::new(
            active_signal_ids.into_iter().collect::<Vec<String>>(),
            supported_sources,
            bothan_version,
            registry_hash,
        );

        self.post(uuid, Topic::Heartbeat, bothan_info).await
    }

    async fn post<T: Serialize>(
        &self,
        uuid: String,
        topic: Topic,
        data: T,
    ) -> Result<Response, Error> {
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
