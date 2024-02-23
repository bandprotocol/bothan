use bothan_binance::Binance;
use bothan_core::service::Service;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::fmt::init;

pub mod price {
    tonic::include_proto!("price"); // Include the generated code.
}

use price::price_service_server::{PriceService, PriceServiceServer}; // Import the generated server module.

pub struct PriceServiceImpl {
    service: Arc<Mutex<Binance>>,
}

impl PriceServiceImpl {
    fn new(binance_service: Binance) -> Self {
        PriceServiceImpl {
            service: Arc::new(Mutex::new(binance_service)),
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

        let mut binance_service = self.service.lock().await;
        let data_list = binance_service.get_price_data(&[&id]).await;
        let mut price_data_list: Vec<price::PriceData> = Vec::new();
        for data in data_list {
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
    if let Ok(service) = Binance::default().await {
        let price_data_impl = PriceServiceImpl::new(service); // Change to mutable binding
        println!("Server running on {}", addr);

        Server::builder()
            .add_service(PriceServiceServer::new(price_data_impl))
            .serve(addr)
            .await?;
    }
    Ok(())
}
