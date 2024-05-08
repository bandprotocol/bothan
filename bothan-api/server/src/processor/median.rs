use std::cmp::Ordering;

use num_traits::{Float, NumCast};
use serde::{Deserialize, Serialize};

use crate::processor::{Processor, ProcessorError};

/// The `MedianProcessor` finds the median of a given data set. It also has a `min_source_count` which
/// is the minimum number of sources required to calculate the median. If the given data set has less
/// than `min_source_count` sources, it returns an error.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MedianProcessor {
    pub min_source_count: usize,
}

impl Processor for MedianProcessor {
    /// Processes the given data and returns the median. If there are not enough sources, it
    /// returns an error.
    fn process(&self, data: Vec<f64>, _: Vec<f64>) -> Result<f64, ProcessorError> {
        if self.min_source_count == 0 {
            return Err(ProcessorError::InvalidParameterValue(
                "min_source_count".to_string(),
            ));
        }

        if data.len() < self.min_source_count {
            return Err(ProcessorError::NotEnoughSources);
        }

        Ok(median(data))
    }
}

fn median<T: Float>(mut data: Vec<T>) -> T {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        let b = data.swap_remove(mid);
        let a = data.swap_remove(mid - 1);
        ((b - a) / NumCast::from(2).unwrap()) + a
    } else {
        data.swap_remove(mid)
    }
}
