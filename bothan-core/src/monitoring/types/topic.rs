use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    Record,
    Heartbeat,
}
