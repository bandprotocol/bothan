use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::Error;
use crate::api::types::{DEFAULT_URL, DEFAULT_USER_AGENT};
use crate::api::CryptoCompareRestAPI;

pub struct CryptoCompareRestAPIBuilder {
    url: String,
    api_key: Option<String>,
    user_agent: String,
}

impl CryptoCompareRestAPIBuilder {
    pub fn set_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    pub fn set_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn set_user_agent(&mut self, user_agent: &str) -> &Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(self) -> Result<CryptoCompareRestAPI, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str(&self.user_agent)?);

        let parsed_url = Url::parse(&self.url)?;

        if let Some(key) = &self.api_key {
            let mut val = HeaderValue::from_str(format!("Apikey {}", key).as_str())?;
            val.set_sensitive(true);
            headers.insert("authorization", val);
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CryptoCompareRestAPI::new(parsed_url, client))
    }
}

impl Default for CryptoCompareRestAPIBuilder {
    fn default() -> Self {
        CryptoCompareRestAPIBuilder {
            url: DEFAULT_URL.to_string(),
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}
