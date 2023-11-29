use crate::error::Error;
use crate::types::Source;

#[async_trait::async_trait]
pub trait Service: Source {
    async fn start(&mut self, symbols: &[&str]) -> Result<(), Error>;
    fn stop(&mut self);
    fn started(&self) -> bool;
}
