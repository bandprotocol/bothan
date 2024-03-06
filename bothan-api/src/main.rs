use bothan_binance::Binance;
use bothan_coingecko::{CoinGeckoService, CoinGeckoServiceBuilder};
use bothan_core::service::Service;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

mod config;
mod proto;

use crate::proto::query::query::query_server::{Query, QueryServer};
use crate::proto::query::query::{PriceData, PriceOption, QueryPricesRequest, QueryPricesResponse};

pub struct PriceServiceImpl {
    binance_service: Arc<Mutex<Binance>>,
    coingecko_service: Arc<Mutex<CoinGeckoService>>,
}

impl PriceServiceImpl {
    fn new(binance_service: Binance, coingecko_service: CoinGeckoService) -> Self {
        PriceServiceImpl {
            binance_service: Arc::new(Mutex::new(binance_service)),
            coingecko_service: Arc::new(Mutex::new(coingecko_service)),
        }
    }
}

#[tonic::async_trait]
impl Query for PriceServiceImpl {
    async fn prices(
        &self, // Change to accept mutable reference
        request: Request<QueryPricesRequest>,
    ) -> Result<Response<QueryPricesResponse>, Status> {
        let symbols = request.into_inner().symbols;

        println!("Received symbols: {:?}", symbols);

        let mut binance_map: HashMap<&str, &str> = HashMap::new();
        let mut coingecko_map: HashMap<&str, &str> = HashMap::new();

        let mut binance_map_reverse: HashMap<&str, &str> = HashMap::new();
        let mut coingecko_map_reverse: HashMap<&str, &str> = HashMap::new();

        binance_map.insert("BTC", "btcusdt");
        binance_map.insert("ETH", "ethusdt");
        coingecko_map.insert("BTC", "bitcoin");
        coingecko_map.insert("ETH", "ethereum");

        binance_map_reverse.insert("BTCUSDT", "BTC");
        binance_map_reverse.insert("ETHUSDT", "ETH");
        coingecko_map_reverse.insert("bitcoin", "BTC");
        coingecko_map_reverse.insert("ethereum", "ETH");

        // TODO: check symbols are not supported before

        let binance_list: &[&str] = &symbols
            .iter()
            .map(|symbol| binance_map.get(symbol.as_str()).unwrap())
            .copied()
            .collect::<Vec<&str>>();

        let coingecko_list: &[&str] = &symbols
            .iter()
            .map(|symbol| coingecko_map.get(symbol.as_str()).unwrap())
            .copied()
            .collect::<Vec<&str>>();

        let mut binance_service = self.binance_service.lock().await;
        let binance_data_list = binance_service.get_price_data(binance_list).await;

        let mut coingecko_service = self.coingecko_service.lock().await;
        let coingecko_data_list = coingecko_service.get_price_data(coingecko_list).await;

        let mut prices: Vec<PriceData> = Vec::new();
        for data_result in binance_data_list {
            let price_data = match data_result {
                Ok(data) => PriceData {
                    // TODO: symbol has to be the symbol from chain side not id
                    symbol: binance_map_reverse
                        .get(data.id.as_str())
                        .unwrap()
                        .to_string(),
                    price: data.price,
                    price_option: PriceOption::Available.into(),
                },
                Err(e) => PriceData {
                    // TODO: logic has to know which symbol is corresponding to the error
                    symbol: e.to_string(),
                    price: "".to_string(),
                    price_option: PriceOption::Unavailable.into(),
                },
            };

            prices.push(price_data.clone())
        }

        for data_result in coingecko_data_list {
            let price_data = match data_result {
                Ok(data) => PriceData {
                    symbol: coingecko_map_reverse
                        .get(data.id.as_str())
                        .unwrap()
                        .to_string(),
                    price: data.price,
                    price_option: PriceOption::Available.into(),
                },
                Err(e) => PriceData {
                    symbol: e.to_string(),
                    price: "".to_string(),
                    price_option: PriceOption::Unavailable.into(),
                },
            };
            prices.push(price_data.clone())
        }

        let response = QueryPricesResponse { prices };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration
    let app_config = config::AppConfig::new().expect("Failed to load configuration");
    println!("{:?}", app_config);

    let addr = app_config.grpc.addr.parse().unwrap();

    let mut coingecko_builder = CoinGeckoServiceBuilder::default();

    let coingecko_config = app_config.source.coingecko;

    if let Some(url) = &coingecko_config.url {
        coingecko_builder = coingecko_builder.set_url(&url);
    }
    if let Some(api_key) = &coingecko_config.api_key {
        coingecko_builder = coingecko_builder.set_api_key(&api_key);
    }
    if let Some(user_agent) = &coingecko_config.user_agent {
        coingecko_builder = coingecko_builder.set_user_agent(&user_agent);
    }
    if let Some(update_interval) = &coingecko_config.update_interval {
        coingecko_builder = coingecko_builder.set_update_interval(*update_interval);
    }
    if let Some(update_supported_assets_interval) =
        &coingecko_config.update_supported_assets_interval
    {
        coingecko_builder = coingecko_builder
            .set_update_supported_assets_interval(*update_supported_assets_interval);
    }
    if let Some(page_size) = &coingecko_config.page_size {
        coingecko_builder = coingecko_builder.set_page_size(*page_size);
    }
    if let Some(page_query_delay) = &coingecko_config.page_query_delay {
        coingecko_builder = coingecko_builder.set_page_query_delay(*page_query_delay);
    }

    let coingecko_service = coingecko_builder.build().await?;

    let binance_service = Binance::default().await?;

    let price_data_impl = PriceServiceImpl::new(binance_service, coingecko_service); // Change to mutable binding

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(QueryServer::new(price_data_impl))
        .serve(addr)
        .await?;

    Ok(())
}
