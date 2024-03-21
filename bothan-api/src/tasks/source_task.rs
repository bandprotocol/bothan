use std::collections::HashSet;

use bothan_core::service::{Service, ServiceResult};
use bothan_core::types::PriceData;

pub struct SourceTask {
    pub source_name: String,
    pub source_ids: HashSet<String>,
}

impl SourceTask {
    pub fn new(source_name: String, source_ids: HashSet<String>) -> Self {
        SourceTask {
            source_name,
            source_ids,
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub async fn get_prices(
        &self,
        service: &mut Box<dyn Service>,
    ) -> Vec<ServiceResult<PriceData>> {
        let ids = self
            .source_ids
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        service.get_price_data(ids.as_slice()).await
    }
}
