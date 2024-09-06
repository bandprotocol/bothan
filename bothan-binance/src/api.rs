pub use error::{ConnectionError, MessageError, SendError};
pub use websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};

pub mod error;
pub mod msgs;
pub mod websocket;
