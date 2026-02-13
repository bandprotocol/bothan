//! Bothan API server forex source configuration.
//!
//! Worker options for supported forex data sources.

use serde::{Deserialize, Serialize};

/// Configuration for the worker sources for forex asset info.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForexSourceConfigs {
    /// Band/kiwi worker options.
    #[serde(default, deserialize_with = "de_kiwi2")]
    pub band_kiwi2: Option<bothan_band::WorkerOpts>,
}

// Macro to generate deserialization functions for Band workers with preset names.
macro_rules! de_band_named {
    ($fn_name:ident, $name:expr) => {
        fn $fn_name<'de, D>(d: D) -> Result<Option<bothan_band::WorkerOpts>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let v = Option::<bothan_band::WorkerOpts>::deserialize(d)?;
            let v = v.map(|w| bothan_band::WorkerOpts::new($name, &w.url, Some(w.update_interval)));
            Ok(v)
        }
    };
}

const BAND1_WORKER_NAME: &str = "band/kiwi2";
de_band_named!(de_kiwi2, BAND1_WORKER_NAME);

impl Default for ForexSourceConfigs {
    fn default() -> Self {
        ForexSourceConfigs {
            band_kiwi2: Some(bothan_band::WorkerOpts::new(
                "band/kiwi2",
                "https://kiwi.bandchain.org",
                None,
            )),
        }
    }
}
