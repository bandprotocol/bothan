pub use error::{ConnectionError, MessageError, SendError};
pub use websocket::{WebSocketConnection, WebSocketConnector};

pub mod error;
pub mod msgs;
pub mod websocket;
