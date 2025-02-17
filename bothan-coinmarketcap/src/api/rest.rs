use std::collections::HashMap;

use crate::api::types::{Quote, Response as CmcResponse};
use crate::worker::error::ProviderError;
use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use itertools::Itertools;
use reqwest::{Client, Url};
use rust_decimal::Decimal;

/// CoinMarketCap REST API client.
pub struct RestApi {
    url: Url,
    client: Client,
}

impl RestApi {
    /// Creates a new CoinMarketCap REST API client.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Fetches the latest quotes for the given cryptocurrency IDs.
    /// Equivalent to the `/v2/cryptocurrency/quotes/latest` endpoint.
    pub async fn get_latest_quotes(
        &self,
        ids: &[usize],
    ) -> Result<Vec<Option<Quote>>, reqwest::Error> {
        let url = format!("{}v2/cryptocurrency/quotes/latest", self.url);
        let ids_string = ids.iter().map(|id| id.to_string()).join(",");
        let params = vec![("id", ids_string)];

        let request_builder = self.client.get(&url).query(&params);
        let response = request_builder.send().await?.error_for_status()?;

        let cmc_response = response
            .json::<CmcResponse<HashMap<String, Quote>>>()
            .await?;
        let quote_map = cmc_response.data;

        let quotes = ids
            .iter()
            .map(|id| quote_map.get(&id.to_string()).cloned())
            .collect();
        Ok(quotes)
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        let int_ids = ids
            .iter()
            .map(|id| {
                id.parse::<usize>()
                    .map_err(|_| ProviderError::InvalidId(id.clone()))
            })
            .collect::<Result<Vec<usize>, _>>()?;

        self.get_latest_quotes(&int_ids)
            .await?
            .into_iter()
            .map(|q| match q {
                None => Err(ProviderError::MissingValue),
                Some(quote) => {
                    let price_float = quote
                        .price_quotes
                        .usd
                        .price
                        .ok_or_else(|| ProviderError::MissingValue)?;
                    let price_dec = Decimal::from_f64_retain(price_float)
                        .ok_or_else(|| ProviderError::InvalidValue)?;
                    let ts = quote
                        .price_quotes
                        .usd
                        .last_updated
                        .parse::<chrono::DateTime<chrono::Utc>>()
                        .map_err(|_| ProviderError::InvalidValue)?
                        .timestamp();
                    let ai = AssetInfo::new(quote.symbol, price_dec, ts);
                    Ok(ai)
                }
            })
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::api::types::{PriceQuote, PriceQuotes, Status};
    use crate::api::RestApiBuilder;
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, RestApi) {
        let server = Server::new_async().await;

        let builder = RestApiBuilder::default()
            .with_url(&server.url())
            .with_api_key("test");
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) fn mock_quote() -> Quote {
        Quote {
            id: 1,
            symbol: "BTC".to_string(),
            slug: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            price_quotes: PriceQuotes {
                usd: PriceQuote {
                    price: Some(80000.0),
                    volume_24h: 123.0,
                    volume_change_24h: 456.0,
                    market_cap: 10000000.0,
                    market_cap_dominance: 99.0,
                    fully_diluted_market_cap: 2100000.0,
                    percent_change_1h: 12.9,
                    percent_change_24h: -70.0,
                    percent_change_7d: -2.0,
                    percent_change_30d: -1.0,
                    last_updated: "2024-03-16T06:55:15.626Z".to_string(),
                },
            },
        }
    }

    pub(crate) trait MockCoinMarketCap {
        fn set_successful_quotes(&mut self, ids: &[&str], quotes: &[Quote]) -> Mock;
        fn set_arbitrary_quotes<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_quotes(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCoinMarketCap for ServerGuard {
        fn set_successful_quotes(&mut self, ids: &[&str], quotes: &[Quote]) -> Mock {
            let response = CmcResponse::<HashMap<String, Quote>> {
                data: quotes
                    .iter()
                    .map(|quote| (quote.id.to_string(), quote.clone()))
                    .collect(),
                status: Status {
                    timestamp: "100000".to_string(),
                    error_code: 0,
                    error_message: None,
                    elapsed: 1,
                    credit_count: 1,
                    notice: None,
                },
            };
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(200)
                .with_body(serde_json::to_string(&response).unwrap())
                .create()
        }

        fn set_arbitrary_quotes<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_quotes(&mut self, ids: &[&str]) -> Mock {
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_quotes() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_quote()];
        let mock = server.set_successful_quotes(&["1"], &quotes);

        let result = client.get_latest_quotes(&[1]).await;
        let expected_result = quotes.into_iter().map(Some).collect::<Vec<Option<Quote>>>();
        mock.assert();
        assert_eq!(result.unwrap(), expected_result);
    }

    #[tokio::test]
    async fn test_successful_get_quotes_with_missing_data() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_quote()];

        let mock = server.set_successful_quotes(&["1", "0"], &quotes);

        let result = client.get_latest_quotes(&[1, 0]).await;

        mock.assert();
        let expected_result = vec![Some(quotes[0].clone()), None];
        assert_eq!(result.unwrap(), expected_result);
    }

    #[tokio::test]
    async fn test_get_quotes_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let mock = server.set_arbitrary_quotes(&["1"], "abc");

        let result = client.get_latest_quotes(&[1]).await;

        mock.assert();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_failed_get_quotes() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_quotes(&["1"]);

        let result = client.get_latest_quotes(&[1]).await;
        mock.assert();
        assert!(result.is_err());
    }
}
