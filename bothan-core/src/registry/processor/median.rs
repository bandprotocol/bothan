use std::ops::{Add, Div, Sub};

use num_traits::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::processor::{Process, ProcessError};

/// The `MedianProcessor` finds the median of a given data set. It also has a `min_source_count` which
/// is the minimum number of sources required to calculate the median. If the given data set has less
/// than `min_source_count` sources, it returns an error.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MedianProcessor {
    pub min_source_count: usize,
}

impl MedianProcessor {
    /// Creates a new `MedianProcessor`.
    pub fn new(min_source_count: usize) -> Self {
        MedianProcessor { min_source_count }
    }
}

impl Process<Decimal> for MedianProcessor {
    /// Processes the given data and returns the median. If there are not enough sources, it
    /// returns an error.
    fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        if self.min_source_count == 0 {
            return Err(ProcessError::new("Minimum source count cannot be zero"));
        }

        if data.len() < self.min_source_count {
            Err(ProcessError::new("Not enough sources to calculate median"))
        } else {
            Ok(median(data))
        }
    }
}

fn median<T>(mut data: Vec<T>) -> T
where
    T: Ord + Copy + FromPrimitive + Add<Output = T> + Div<Output = T> + Sub<Output = T>,
{
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        let b = data.swap_remove(mid);
        let a = data.swap_remove(mid - 1);
        ((b - a) / T::from_u8(2).unwrap()) + a
    } else {
        data.swap_remove(mid)
    }
}
