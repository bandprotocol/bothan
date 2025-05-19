/// Asset information data structure for storing price and timestamp data.
///
/// This module provides the [`AssetInfo`] struct which is the core data structure
/// for representing asset price information throughout the system.
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents price and timestamp information for a specific asset.
///
/// `AssetInfo` stores the essential information about an asset's current state,
/// including its unique identifier, price, and the timestamp of when the information
/// was last updated. This structure is used throughout the system for storing,
/// transmitting, and displaying asset information.
///
/// The structure implements various traits for serialization, deserialization,
/// and display, making it compatible with different storage backends and
/// presentation formats.
///
/// # Examples
///
/// ```rust
/// use rust_decimal::Decimal;
/// use bothan_lib::types::AssetInfo;
///
/// // Create a new asset information entry
/// let asset = AssetInfo::new(
///     "BTC-USD".to_string(),
///     Decimal::new(3950000, 2), // $39,500.00
///     1634567890000, // Unix timestamp in milliseconds
/// );
///
/// // Access the asset information
/// assert_eq!(asset.id, "BTC-USD");
/// assert_eq!(asset.price.to_string(), "39500.00");
/// ```
#[derive(Clone, PartialEq, Debug, Display, Serialize, Deserialize)]
#[display("AssetInfo{{id: {id}, price: {price}, timestamp: {timestamp}}}")]
pub struct AssetInfo {
    /// Unique identifier for the asset, typically in the format of a trading pair (e.g., "BTC-USD").
    pub id: String,

    /// Current price of the asset represented as a high-precision decimal.
    pub price: Decimal,

    /// Unix timestamp (in milliseconds) when the asset information was last updated.
    pub timestamp: i64,
}

impl AssetInfo {
    /// Creates a new AssetInfo instance with the specified id, price, and timestamp.
    ///
    /// This is the recommended way to construct a new AssetInfo instance,
    /// ensuring all fields are properly initialized.
    pub fn new(id: String, price: Decimal, timestamp: i64) -> Self {
        Self {
            id,
            price,
            timestamp,
        }
    }
}

/// Custom binary encoding implementation for efficient serialization.
///
/// This implementation ensures that AssetInfo can be efficiently encoded
/// in a binary format, which is important for storage and network transmission.
impl Encode for AssetInfo {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.id, encoder)?;
        Encode::encode(&self.price.serialize(), encoder)?;
        Encode::encode(&self.timestamp, encoder)
    }
}

/// Custom binary decoding implementation for efficient deserialization.
///
/// This implementation ensures that AssetInfo can be efficiently decoded
/// from a binary format, which is important when retrieving from storage
/// or receiving over the network.
impl<Context> Decode<Context> for AssetInfo {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let id: String = Decode::decode(decoder)?;
        let price_serialized: [u8; 16] = Decode::decode(decoder)?;
        let timestamp: i64 = Decode::decode(decoder)?;

        Ok(AssetInfo {
            id,
            price: Decimal::deserialize(price_serialized),
            timestamp,
        })
    }
}
