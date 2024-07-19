use std::collections::HashMap;
use std::fmt::Display;

use crate::registry::Registry;

pub fn into_key<T: Display, U: Display>(source_id: &T, id: &U) -> String {
    format!("{}-{}", source_id, id)
}

// TODO: Change function name
pub fn find_diff(set: Vec<&String>, registry: &Registry) -> HashMap<String, Vec<String>> {
    set.into_iter().fold(HashMap::new(), |mut acc, signal_id| {
        if let Some(signal) = registry.get(signal_id) {
            signal.source_queries.iter().for_each(|source| {
                acc.entry(source.source_id.clone())
                    .or_default()
                    .push(source.query_id.clone());
            });
        }
        acc
    })
}

#[macro_export]
macro_rules! price {
    ($signal_id:expr, $status:expr, $price:expr) => {
        Price {
            signal_id: $signal_id.clone(),
            status: $status.into(),
            price: $price,
        }
    };
}
