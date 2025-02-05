use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Display, Serialize, Deserialize)]
#[display("AssetInfo{{id: {id}, price: {price}, timestamp: {timestamp}}}")]
pub struct AssetInfo {
    pub id: String,
    pub price: Decimal,
    pub timestamp: i64,
}

impl AssetInfo {
    pub fn new(id: String, price: Decimal, timestamp: i64) -> Self {
        Self {
            id,
            price,
            timestamp,
        }
    }
}

impl Encode for AssetInfo {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.id, encoder)?;
        Encode::encode(&self.price.serialize(), encoder)?;
        Encode::encode(&self.timestamp, encoder)
    }
}

impl Decode for AssetInfo {
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
