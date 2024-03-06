use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinanceServiceConfig {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub rem_id_ch_size: Option<usize>,
}
