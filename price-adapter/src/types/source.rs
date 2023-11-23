use crate::error::Error;
use crate::types::{PriceInfo, WebsocketMessage};
use futures_util::{stream::FusedStream, StreamExt};

#[async_trait::async_trait]
/// Represents a source for fetching prices through HTTP requests.
///
/// This trait defines methods for obtaining price information for a given set
/// of symbols using HTTP requests. Implementors are expected to provide an
/// asynchronous implementation for retrieving prices.
pub trait HttpSource: Send + Sync + 'static {
    /// Asynchronously retrieves prices for the specified symbols.
    ///
    /// This method takes a slice of symbol strings and returns a vector of
    /// `Result<PriceInfo, Error>`. Each result represents the outcome of
    /// attempting to fetch price information for a specific symbol.
    async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>>;
}

#[async_trait::async_trait]
/// Represents a source for streaming WebSocket messages.
///
/// This trait defines methods for connecting to a WebSocket, subscribing and
/// unsubscribing to symbols, checking the connection status, and streaming
/// WebSocket messages.
pub trait WebSocketSource:
    Send + Sync + StreamExt<Item = Result<WebsocketMessage, Error>> + FusedStream + Unpin + 'static
{
    /// Asynchronously establishes a connection to the WebSocket.
    async fn connect(&mut self) -> Result<(), Error>;

    /// Asynchronously subscribes to the specified symbols on the WebSocket.
    ///
    /// Returns the number of symbols successfully subscribed.
    async fn subscribe(&mut self, symbols: &[&str]) -> Result<u32, Error>;

    /// Asynchronously unsubscribes from the specified symbols on the WebSocket.
    ///
    /// Returns the number of symbols successfully unsubscribed.
    async fn unsubscribe(&mut self, symbols: &[&str]) -> Result<u32, Error>;

    /// Checks whether the WebSocket is currently connected.
    fn is_connected(&self) -> bool;
}
