use bothan_binance::Binance;
use bothan_coingecko::{CoinGeckoService, CoinGeckoServiceBuilder};
use bothan_core::service::Service;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

// pub mod price {
//     tonic::include_proto!("price"); // Include the generated code.
// }

// use price::price_service_server::{PriceService, PriceServiceServer}; // Import the generated server module.

pub mod price;

use crate::price::price::price_service_server::{PriceService, PriceServiceServer};
use crate::price::price::{PriceData, PricesRequest, PricesResponse};

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
impl PriceService for PriceServiceImpl {
    async fn get_prices(
        &self, // Change to accept mutable reference
        request: Request<PricesRequest>,
    ) -> Result<Response<PricesResponse>, Status> {
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

        binance_map_reverse.insert("btcusdt", "BTC");
        binance_map_reverse.insert("ethusdt", "ETH");
        coingecko_map_reverse.insert("bitcoin", "BTC");
        coingecko_map_reverse.insert("ethereum", "ETH");

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
                    symbol: data.id,
                    price: data.price,
                    error: "".to_string(),
                },
                Err(e) => PriceData {
                    // TODO: logic has to know which symbol is corresponding to the error
                    symbol: e.to_string(),
                    price: "".to_string(),
                    error: e.to_string(),
                },
            };

            prices.push(price_data.clone())
        }

        for data_result in coingecko_data_list {
            let price_data = match data_result {
                Ok(data) => PriceData {
                    symbol: data.id,
                    price: data.price,
                    error: "".to_string(),
                },
                Err(e) => PriceData {
                    symbol: e.to_string(),
                    price: "".to_string(),
                    error: e.to_string(),
                },
            };
            prices.push(price_data.clone())
        }

        let response = PricesResponse { prices };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service_result = CoinGeckoServiceBuilder::default()
        .set_update_supported_assets_interval(Duration::from_secs(600))
        .build()
        .await;

    if let Ok(service) = Binance::default().await {
        if let Ok(coingecko_service) = service_result {
            let price_data_impl = PriceServiceImpl::new(service, coingecko_service); // Change to mutable binding

            println!("Server running on {}", addr);

            Server::builder()
                .add_service(PriceServiceServer::new(price_data_impl))
                .serve(addr)
                .await?;
        }
    }
    Ok(())
}
