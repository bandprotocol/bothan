use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::Error;
use crate::api::types::{DEFAULT_PRO_URL, DEFAULT_URL, DEFAULT_USER_AGENT};
use crate::api::CoinGeckoRestAPI;

pub struct CoinGeckoRestAPIBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: String,
}

impl CoinGeckoRestAPIBuilder {
    pub fn with_url(&mut self, url: &str) -> &Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_user_agent(&mut self, user_agent: &str) -> &Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(self) -> Result<CoinGeckoRestAPI, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str(&self.user_agent)?);

        let url = match self.url {
            Some(url) => url,
            None => match &self.api_key {
                None => DEFAULT_URL.to_string(),
                Some(_) => DEFAULT_PRO_URL.to_string(),
            },
        };
        let parsed_url = Url::parse(&url)?;

        if let Some(key) = &self.api_key {
            let mut val = HeaderValue::from_str(key)?;
            val.set_sensitive(true);
            headers.insert("x-cg-pro-api-key", val);
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CoinGeckoRestAPI::new(parsed_url, client))
    }
}

impl Default for CoinGeckoRestAPIBuilder {
    fn default() -> Self {
        CoinGeckoRestAPIBuilder {
            url: None,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}
