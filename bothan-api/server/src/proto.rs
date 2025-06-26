//! # Protocol Buffer Bindings and API Types
//!
//! This module contains the generated protocol buffer bindings and
//! additional API types for the Bothan API server.
//!
//! ## Notes
//!
//! - This file includes generated code and should not be edited manually.
//! - Additional helper methods and trait implementations are provided for API types.

#![allow(clippy::all)]
pub mod bothan {
    pub mod v1 {
        pub use bothan_service_server::{BothanService, BothanServiceServer};
        use serde::Serialize;
        use serde::ser::SerializeStruct;

        include!("proto/bothan.v1.rs");

        pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("proto/descriptor.pb");

        impl Price {
            /// Creates a new `Price` instance for API responses.
            pub fn new<T: Into<String>, U: Into<u64>>(
                signal_id: T,
                price: U,
                status: Status,
            ) -> Price {
                Price {
                    signal_id: signal_id.into(),
                    price: price.into(),
                    status: status.into(),
                }
            }
        }

        impl Serialize for Price {
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
    }
}
