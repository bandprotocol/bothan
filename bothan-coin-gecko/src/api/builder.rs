use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::Error;
use crate::api::types::{DEFAULT_PRO_URL, DEFAULT_URL, DEFAULT_USER_AGENT};
use crate::api::CoinGeckoRestAPI;

#[derive(Default)]
pub struct CoinGeckoRestAPIBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: Option<String>,
}

impl CoinGeckoRestAPIBuilder {
    pub fn set_url(&mut self, url: &str) -> &Self {
        self.url = Some(url.into());
        self
    }

    pub fn set_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn set_user_agent(&mut self, user_agent: &str) -> &Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn build(self) -> Result<CoinGeckoRestAPI, Error> {
        let mut headers = HeaderMap::new();

        let user_agent = match &self.user_agent {
            Some(user_agent) => user_agent,
            None => DEFAULT_USER_AGENT,
        };
        headers.insert("User-Agent", HeaderValue::from_str(user_agent)?);

        if let Some(key) = &self.api_key {
            let mut val = HeaderValue::from_str(key)?;
            val.set_sensitive(true);
            headers.insert("x-cg-pro-api-key", val);
        }

        let url = match &self.url {
            Some(url) => url,
            None => match &self.api_key {
                Some(_) => DEFAULT_PRO_URL,
                None => DEFAULT_URL,
            },
        };

        let parsed_url = Url::parse(url)?;
        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CoinGeckoRestAPI::new(parsed_url, client))
    }
}
