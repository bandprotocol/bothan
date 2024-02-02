use crate::websocket::types::MiniTickerInfo;
use crate::websocket::types::MiniTickerResponse;

pub fn parse_message(msg: String) -> Option<MiniTickerInfo> {
    if let Ok(resp) = serde_json::from_str::<MiniTickerResponse>(&msg) {
        Some(resp.data)
    } else {
        None
    }
}