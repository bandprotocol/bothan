use std::collections::HashSet;

use bothan_core::service::{Service, ServiceResult};
use bothan_core::types::PriceData;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceTask {
    source_name: String,
    source_ids: HashSet<String>,
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

    pub fn source_ids(&self) -> Vec<&str> {
        self.source_ids.iter().map(|s| s.as_str()).collect()
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
