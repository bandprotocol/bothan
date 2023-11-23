use crate::error::Error;
use serde_json::Value;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait Mapper: Send + Sync + Sized + Unpin + 'static {
    /// Asynchronously retrieves the mapping data.
    ///
    /// This method is responsible for fetching and returning the mapping data,
    /// which is represented as a `HashMap<String, Value>`. It returns a `Result`
    /// containing the mapping data or an `Error` if the operation fails.
    async fn get_mapping(&self) -> Result<&HashMap<String, Value>, Error>;
}
