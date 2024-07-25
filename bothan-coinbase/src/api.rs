pub use error::{ConnectionError, MessageError, SendError};
pub use websocket::{CoinbaseWebSocketConnection, CoinbaseWebSocketConnector};

pub mod error;
pub mod types;
pub mod websocket;
