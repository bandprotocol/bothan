pub use error::{ConnectionError, MessageError, SendError};
pub use websocket::{KrakenWebSocketConnection, KrakenWebSocketConnector};

pub mod error;
pub mod types;
pub mod websocket;
