use serde::ser::SerializeStruct;
use serde::Serialize;

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod price;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod signal;

impl price::Price {
    pub fn new<T: Into<String>, U: Into<u64>>(
        signal_id: T,
        price: U,
        status: price::Status,
    ) -> price::Price {
        price::Price {
            signal_id: signal_id.into(),
            price: price.into(),
            status: status.into(),
        }
    }
}

impl Serialize for price::Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Price", 3)?;
        s.serialize_field("signal_id", &self.signal_id)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("status", &self.status)?;
        s.end()
    }
}
