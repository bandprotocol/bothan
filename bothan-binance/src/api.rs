pub use error::{ConnectionError, MessageError, SubscriptionError};
pub use websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};

pub mod error;
pub mod msgs;
pub mod websocket;
