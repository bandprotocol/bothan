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
            use serde::Deserialize;
            let v = Option::<bothan_band::WorkerOpts>::deserialize(d)?;
            let v = v.map(|w| bothan_band::WorkerOpts::new($name, &w.url, Some(w.update_interval)));
            Ok(v)
        }
    };
}

pub(crate) use de_band_named;
