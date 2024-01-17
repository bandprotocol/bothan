use crate::error::Error;
use crate::types::Source;

#[async_trait::async_trait]
pub trait Service: Source {
    async fn start(&mut self, symbols: &[&str]) -> Result<(), Error>;
    async fn stop(&mut self);
    async fn is_started(&self) -> bool;
}
