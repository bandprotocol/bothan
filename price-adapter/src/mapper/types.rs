use crate::error::Error;
use serde_json::Value;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait Mapper: Send + Sync + Sized + Unpin + 'static {
    async fn get_mapping(&self) -> Result<&HashMap<String, Value>, Error>;
}
