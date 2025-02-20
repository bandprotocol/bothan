use super::topic::Topic;
use serde::Serialize;

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
