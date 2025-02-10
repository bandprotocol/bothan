use crate::api::error::ProviderError;
use crate::api::msg::ticker::Ticker;
use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use reqwest::{Client, Url};
use rust_decimal::Decimal;

pub const DEFAULT_URL: &str = "https://api-pub.bitfinex.com/v2/";

pub struct RestApi {
    url: Url,
    client: Client,
}

impl RestApi {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_tickers<T: AsRef<str>>(
        &self,
        tickers: &[T],
    ) -> Result<Vec<Ticker>, reqwest::Error> {
        let url = format!("{}/tickers", self.url);
        let symbols = tickers
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<&str>>()
            .join(",");
        let params = vec![("symbols", symbols)];

        let resp = self.client.get(&url).query(&params).send().await?;
        resp.error_for_status_ref()?;
        resp.json().await
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        let timestamp = chrono::Utc::now().timestamp();
        self.get_tickers(ids)
            .await?
            .into_iter()
            .map(|t| {
                let price =
                    Decimal::from_f64_retain(t.price()).ok_or(ProviderError::InvalidValue)?;
                Ok(AssetInfo::new(t.symbol().to_string(), price, timestamp))
            })
            .collect()
    }
}
