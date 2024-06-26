use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::manager::PriceServiceManager;
use crate::proto::query::query_server::Query;
use crate::proto::query::{QueryPricesRequest, QueryPricesResponse};
use crate::utils::arc_mutex;

/// The `CryptoQueryServer` struct represents a server for querying cryptocurrency prices.
pub struct CryptoQueryServer {
    manager: Arc<Mutex<PriceServiceManager>>,
}

impl CryptoQueryServer {
    /// Creates a new `CryptoQueryServer` instance.
    pub fn new(manager: PriceServiceManager) -> Self {
        CryptoQueryServer {
            manager: arc_mutex!(manager),
        }
    }
}

#[tonic::async_trait]
impl Query for CryptoQueryServer {
    async fn prices(
        &self, // Change to accept mutable reference
        request: Request<QueryPricesRequest>,
    ) -> Result<Response<QueryPricesResponse>, Status> {
        let signal_ids = request.into_inner().signal_ids;
        info!("crypto_price::received::{:?}", signal_ids);
        let l = &signal_ids
            .iter()
            .map(|symbol| symbol.as_str())
            .collect::<Vec<&str>>();

        let mut manager = self.manager.lock().await;
        let prices = manager.get_prices(l).await;

        let response = QueryPricesResponse { prices };
        info!("crypto_price::response::{:?}", response);
        Ok(Response::new(response))
    }
}
