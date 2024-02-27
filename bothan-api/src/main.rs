use bothan_binance::Binance;
use bothan_coingecko::{CoinGeckoService, CoinGeckoServiceBuilder};
use bothan_core::service::Service;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

pub mod price {
    tonic::include_proto!("price"); // Include the generated code.
}

use price::price_service_server::{PriceService, PriceServiceServer}; // Import the generated server module.

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
    async fn get_price_data(
        &self, // Change to accept mutable reference
        request: Request<price::PriceDataRequest>,
    ) -> Result<Response<price::PriceDataResponse>, Status> {
        let id = request.into_inner().id;

        // Here you can call your `get_price_data` function with the received `id`.
        // For this example, let's just print it.
        println!("Received id: {}", id);

        let mut binance_map: HashMap<&str, &str> = HashMap::new();
        let mut coingecko_map: HashMap<&str, &str> = HashMap::new();

        binance_map.insert("BTC", "btcusdt");
        coingecko_map.insert("BTC", "bitcoin");

        let mut binance_service = self.binance_service.lock().await;
        let binance_data_list = binance_service
            .get_price_data(&[binance_map.get(id.as_str()).unwrap()])
            .await;

        let mut coingecko_service = self.coingecko_service.lock().await;
        let coingecko_data_list = coingecko_service
            .get_price_data(&[coingecko_map.get(id.as_str()).unwrap()])
            .await;

        let mut price_data_list: Vec<price::PriceData> = Vec::new();
        for data in binance_data_list {
            let price_data = data.unwrap();
            let price_data = price::PriceData {
                id: price_data.id,
                price: price_data.price,
                timestamp: price_data.timestamp,
            };
            price_data_list.push(price_data.clone())
        }

        for data in coingecko_data_list {
            let price_data = data.unwrap();
            let price_data = price::PriceData {
                id: price_data.id,
                price: price_data.price,
                timestamp: price_data.timestamp,
            };
            price_data_list.push(price_data.clone())
        }

        let response = price::PriceDataResponse { price_data_list };

        // Simulating some response data

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
        if let Ok(mut coingecko_service) = service_result {
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
