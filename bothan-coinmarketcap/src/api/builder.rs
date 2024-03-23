use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError as Error;
use crate::api::types::DEFAULT_URL;
use crate::api::CoinMarketCapRestAPI;

pub struct CoinMarketCapRestAPIBuilder {
    url: String,
    api_key: Option<String>,
}

impl CoinMarketCapRestAPIBuilder {
    pub fn set_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    pub fn set_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn build(self) -> Result<CoinMarketCapRestAPI, Error> {
        let mut headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        let key = match &self.api_key {
            Some(key) => key,
            None => return Err(Error::MissingAPIKey()),
        };

        let mut val = HeaderValue::from_str(key)?;
        val.set_sensitive(true);
        headers.insert("X-CMC_PRO_API_KEY", val);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CoinMarketCapRestAPI::new(parsed_url, client))
    }
}

impl Default for CoinMarketCapRestAPIBuilder {
    fn default() -> Self {
        CoinMarketCapRestAPIBuilder {
            url: DEFAULT_URL.into(),
            api_key: None,
        }
    }
}
