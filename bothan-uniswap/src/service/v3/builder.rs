use std::time::Duration;

use alloy::network::Ethereum;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::Http;
#[cfg(feature = "hyper")]
use alloy::{providers::HyperProvider, transports::http::HyperClient};
#[cfg(feature = "reqwest")]
use alloy::{providers::ReqwestProvider, transports::http::Client};
use serde::Deserialize;
use url::Url;

use crate::service::v3::error::Error;
use crate::service::v3::types::{DEFAULT_INVERSE_SETTING, DEFAULT_UPDATE_INTERVAL};
use crate::service::v3::UniswapV3Service;

#[cfg(feature = "reqwest")]
pub type UniswapV3ReqwestService = UniswapV3Service<ReqwestProvider, Http<Client>, Ethereum>;

#[cfg(feature = "hyper")]
pub type UniswapV3HyperService = UniswapV3Service<HyperProvider, Http<HyperClient>, Ethereum>;

#[derive(Clone, Debug, Deserialize)]
pub struct UniswapV3ServiceBuilderOpts {
    pub provider_url: String,
    pub update_interval: Option<Duration>,
    pub inverse: Option<bool>,
}

pub struct UniswapV3ServiceBuilder {
    provider_url: String,
    update_interval: Duration,
    inverse: bool,
}

impl UniswapV3ServiceBuilder {
    pub fn new(opts: UniswapV3ServiceBuilderOpts) -> Self {
        Self {
            provider_url: opts.provider_url,
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
            inverse: opts.inverse.unwrap_or(DEFAULT_INVERSE_SETTING),
        }
    }

    pub fn with_provider(mut self, url: &str) -> Self {
        self.provider_url = url.to_string();
        self
    }

    pub fn update_interval(mut self, interval: Duration) -> Self {
        self.update_interval = interval;
        self
    }

    #[cfg(feature = "reqwest")]
    pub async fn build_with_reqwest_http(
        self,
    ) -> Result<UniswapV3Service<ReqwestProvider, Http<Client>, Ethereum>, Error> {
        let url = Url::parse(&self.provider_url)?;
        let provider = ProviderBuilder::new().on_http(url);
        let _ = provider.get_block_number().await?;
        let service = UniswapV3Service::new(provider, self.update_interval, self.inverse).await;
        Ok(service)
    }

    #[cfg(feature = "hyper")]
    pub async fn build_with_hyper_http(
        self,
    ) -> Result<UniswapV3Service<HyperProvider, Http<HyperClient>, Ethereum>, Error> {
        let provider = ProviderBuilder::new().on_hyper_http(Url::parse(&self.provider_url)?);
        let _ = provider.get_block_number().await?;
        let service = UniswapV3Service::new(provider, self.update_interval, self.inverse).await;
        Ok(service)
    }
}

impl Default for UniswapV3ServiceBuilder {
    fn default() -> Self {
        Self {
            provider_url: "".to_string(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
            inverse: DEFAULT_INVERSE_SETTING,
        }
    }
}
