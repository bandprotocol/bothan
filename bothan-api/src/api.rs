use std::sync::Arc;

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query::query_server::Query;
use crate::proto::query::query::{QueryPricesRequest, QueryPricesResponse};
use crate::utils::arc_mutex;

/// `APIServiceImpl` is the implementation of the Query service to be used by the gRPC server to
/// query prices.
pub struct APIServiceImpl {
    manager: Arc<Mutex<PriceServiceManager>>,
}

impl APIServiceImpl {
    /// Creates a new `APIServiceImpl` with the given `PriceServiceManager`.
    pub fn new(manager: PriceServiceManager) -> Self {
        APIServiceImpl {
            manager: arc_mutex!(manager),
        }
    }
}

#[tonic::async_trait]
impl Query for APIServiceImpl {
    /// Queries and returns the price of the given signals
    async fn prices(
        &self, // Change to accept mutable reference
        request: Request<QueryPricesRequest>,
    ) -> Result<Response<QueryPricesResponse>, Status> {
        let signal_ids = request.into_inner().signal_ids;
        println!("Received signal_ids: {:?}", signal_ids);
        let l = &signal_ids
            .iter()
            .map(|symbol| symbol.as_str())
            .collect::<Vec<&str>>();

        let mut manager = self.manager.lock().await;
        let prices = manager.get_prices(l).await;

        let response = QueryPricesResponse { prices };

        Ok(Response::new(response))
    }
}
