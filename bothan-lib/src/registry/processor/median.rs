// ! Median processor for combining source data values.
//!
//! This module provides a processor that calculates the median of multiple input values.
//! Median calculation provides a robust measure of central tendency that is less affected
//! by outliers compared to the arithmetic mean.
//!
//! The module provides:
//!
//! - The [`MedianProcessor`] struct which implements median calculation
//! - Support for a minimum source count requirement
//! - Proper handling of both odd and even number of data points
//!
//! # Median Calculation
//!
//! The median calculation follows these rules:
//!
//! - For odd number of values, the median is the middle value after sorting
//! - For even number of values, the median is the average of the two middle values
//! - Processing fails if fewer than the required minimum sources are available

use std::cmp::max;
use std::ops::{Add, Div, Sub};

use bincode::{Decode, Encode};
use num_traits::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::processor::ProcessError;

/// Processor that calculates the median of input values.
///
/// The `MedianProcessor` computes the median of a set of input values, requiring a minimum
/// number of data sources to be present. This provides a robust measure of central tendency
/// that is resistant to outliers in the data.
///
/// # Minimum Source Count
///
/// The processor requires at least `min_source_count` data points to calculate a median.
/// If fewer data points are provided, processing will fail with an error.
///
/// # Median Calculation
///
/// - For odd number of values, returns the middle value after sorting
/// - For even number of values, returns the average of the two middle values
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::processor::{Processor, median::MedianProcessor};
/// use rust_decimal::Decimal;
///
/// // Create a median processor that requires at least 3 sources
/// let processor = Processor::Median(MedianProcessor { min_source_count: 3 });
///
/// // Process some data with 5 values
/// let data = vec![
///     ("source1".to_string(), Decimal::new(100, 0)),
///     ("source2".to_string(), Decimal::new(200, 0)),
///     ("source3".to_string(), Decimal::new(300, 0)),
///     ("source4".to_string(), Decimal::new(400, 0)),
///     ("source5".to_string(), Decimal::new(500, 0)),
/// ];
///
/// let result = processor.process(data).unwrap();
/// assert_eq!(result, Decimal::new(300, 0)); // Median is 300
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct MedianProcessor {
    /// The minimum number of sources required to calculate the median.
    ///
    /// If fewer than this number of data points are provided, the processing
    /// will fail with an error.
    pub min_source_count: usize,
}

impl MedianProcessor {
    /// Creates a new `MedianProcessor` with the specified minimum source count.
    ///
    /// The minimum source count determines how many data points are required at minimum
    /// to calculate a valid median. This helps ensure the result has sufficient statistical
    /// significance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::processor::median::MedianProcessor;
    ///
    /// // Create a processor that requires at least 3 sources
    /// let processor = MedianProcessor::new(3);
    /// ```
    pub fn new(min_source_count: usize) -> Self {
        MedianProcessor { min_source_count }
    }

    /// Calculates the median of the provided decimal values.
    ///
    /// This method computes the median of the input values, failing if there are fewer
    /// data points than the required minimum.
    ///
    /// # Errors
    ///
    /// Returns a `ProcessError` if there are fewer than `min_source_count` data points.
    pub fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        if data.len() < max(self.min_source_count, 1) {
            Err(ProcessError::new("Not enough sources to calculate median"))
        } else {
            Ok(median(data))
        }
    }
}

/// Calculates the median of a vector of values.
///
/// This function computes the median of the provided values:
/// - For an odd number of elements, returns the middle element after sorting
/// - For an even number of elements, returns the average of the two middle elements
///
/// The function is generic and works with any type that supports the required operations.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_with_odd_count() {
        let median = MedianProcessor::new(1);
        let data = vec![
            Decimal::from(510),
            Decimal::from(202),
            Decimal::from(10),
            Decimal::from(4300),
            Decimal::from(501),
        ];
        let res = median.process(data);

        assert_eq!(res.unwrap(), Decimal::from(501));
    }

    #[test]
    fn test_median_with_even_count() {
        let median = MedianProcessor::new(1);
        let data = vec![
            Decimal::from(101000878),
            Decimal::from(210),
            Decimal::from(333120),
            Decimal::from(4000),
            Decimal::from(50120),
            Decimal::from(50122),
        ];
        let res = median.process(data);

        assert_eq!(res.unwrap(), Decimal::from(50121));
    }
}
