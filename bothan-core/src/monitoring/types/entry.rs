//! Bothan core monitoring entry type.
//!
//! Defines the `Entry` struct for monitoring messages.

use serde::Serialize;

use super::topic::Topic;

#[derive(Serialize)]
pub struct Entry<T: Serialize> {
    pub uuid: String,
    pub topic: Topic,
    pub data: T,
}

impl<T: Serialize> Entry<T> {
    pub fn new(uuid: String, topic: Topic, data: T) -> Entry<T> {
        Entry { uuid, topic, data }
    }
}
