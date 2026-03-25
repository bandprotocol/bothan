//! Bothan API server forex source configuration.
//!
//! Worker options for supported forex data sources.

use serde::{Deserialize, Serialize};

/// Configuration for the worker sources for forex asset info.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForexSourceConfigs {
    /// Band/owlet worker options.
    ///
    /// NOTE: The `name` field in `WorkerOpts` is marked with `#[serde(skip)]`, so deserialized instances
    /// will have an empty/default name. The custom deserializer `de_owlet` reconstructs the options
    #[serde(default, deserialize_with = "de_owlet")]
    pub band_owlet: Option<bothan_band::WorkerOpts>,
    /// Band/fieldfare worker options.
    ///
    /// NOTE: The `name` field in `WorkerOpts` is marked with `#[serde(skip)]`, so deserialized instances
    /// will have an empty/default name. The custom deserializer `de_fieldfare` reconstructs the options
    #[serde(default, deserialize_with = "de_fieldfare")]
    pub band_fieldfare: Option<bothan_band::WorkerOpts>,
    /// Band/xenops worker options.
    ///
    /// NOTE: The `name` field in `WorkerOpts` is marked with `#[serde(skip)]`, so deserialized instances
    /// will have an empty/default name. The custom deserializer `de_xenops` reconstructs the options
    #[serde(default, deserialize_with = "de_xenops")]
    pub band_xenops: Option<bothan_band::WorkerOpts>,
}

// Macro to generate deserialization functions for Band workers with preset names.
// This macro defines a function that:
// - Deserializes an Option<WorkerOpts>,
// - If present, creates a new WorkerOpts with the given name and original URL/update_interval.
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

const BAND1_WORKER_NAME: &str = "band/owlet";
de_band_named!(de_owlet, BAND1_WORKER_NAME);

const BAND2_WORKER_NAME: &str = "band/fieldfare";
de_band_named!(de_fieldfare, BAND2_WORKER_NAME);

const BAND3_WORKER_NAME: &str = "band/xenops";
de_band_named!(de_xenops, BAND3_WORKER_NAME);

impl Default for ForexSourceConfigs {
    fn default() -> Self {
        ForexSourceConfigs {
            band_owlet: Some(bothan_band::WorkerOpts::new(
                "band/owlet",
                "https://owlet.bandchain.org",
                None,
            )),
            band_fieldfare: Some(bothan_band::WorkerOpts::new(
                "band/fieldfare",
                "https://fieldfare.bandchain.org",
                None,
            )),
            band_xenops: Some(bothan_band::WorkerOpts::new(
                "band/xenops",
                "https://xenops.bandchain.org",
                None,
            )),
        }
    }
}
