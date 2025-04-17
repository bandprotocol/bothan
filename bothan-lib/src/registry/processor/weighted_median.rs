// ! Weighted median processor for combining source data values.
//!
//! This module provides a processor that calculates the weighted median of multiple input values.
//! Unlike a standard median, a weighted median takes into account the relative importance
//! (weight) of each data source when determining the central value.
//!
//! The module provides:
//!
//! - The [`WeightedMedianProcessor`] struct which implements weighted median calculation
//! - Support for source-specific weights configuration
//! - Minimum cumulative weight requirement for statistical significance
//!
//! # Weighted Median Calculation
//!
//! The weighted median calculation:
//!
//! - Assigns weights to each data source
//! - Sorts the values in ascending order
//! - Finds the value where the cumulative weight crosses the mid-point threshold
//! - Handles special cases where the mid-point falls exactly between two values
//!
//! This approach gives more influence to sources with higher weights, which is useful
//! when some data sources are considered more reliable or important than others.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Div};

use bincode::{Decode, Encode};
use num_traits::{FromPrimitive, Zero};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::processor::ProcessError;

/// Processor that calculates the weighted median of input values.
///
/// The `WeightedMedianProcessor` computes the weighted median of a set of input values,
/// taking into account the relative importance (weight) of each data source. This provides
/// a way to give more influence to more reliable or important sources when determining
/// the central value.
///
/// # Source Weights
///
/// Each source is assigned a weight using the `source_weights` mapping. Sources with higher
/// weights have more influence on the resulting median value. Sources that are not present
/// in the mapping will cause processing to fail.
///
/// # Minimum Cumulative Weight
///
/// The processor requires the sum of weights from all provided sources to meet or exceed
/// the `minimum_cumulative_weight` threshold. This ensures that enough reliable data is
/// present to calculate a statistically significant result.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use bothan_lib::registry::processor::{Processor, weighted_median::WeightedMedianProcessor};
/// use rust_decimal::Decimal;
///
/// // Create weights for different sources
/// let weights = HashMap::from([
///     ("source1".to_string(), 10),
///     ("source2".to_string(), 20),
///     ("source3".to_string(), 30),
/// ]);
///
/// // Create a weighted median processor
/// let processor = Processor::WeightedMedian(WeightedMedianProcessor {
///     source_weights: weights,
///     minimum_cumulative_weight: 50,
/// });
///
/// // Process data with different weights
/// let data = vec![
///     ("source1".to_string(), Decimal::new(100, 0)), // Weight: 10
///     ("source2".to_string(), Decimal::new(200, 0)), // Weight: 20
///     ("source3".to_string(), Decimal::new(300, 0)), // Weight: 30
/// ];
///
/// let result = processor.process(data).unwrap();
/// // The result is biased toward the value from source3 since it has the highest weight
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct WeightedMedianProcessor {
    /// Mapping of source identifiers to their respective weights.
    ///
    /// Each source is assigned a weight, which determines its influence on the
    /// resulting median calculation. Sources with higher weights have more impact
    /// on the final result.
    pub source_weights: HashMap<String, u32>,
    
    /// The minimum sum of weights required for a valid calculation.
    ///
    /// Processing will fail if the sum of weights from all provided sources
    /// does not meet this threshold. This ensures sufficient data quality
    /// and statistical significance.
    pub minimum_cumulative_weight: u32,
}

impl WeightedMedianProcessor {
    /// Creates a new `WeightedMedianProcessor` with the specified weights and minimum threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use bothan_lib::registry::processor::weighted_median::WeightedMedianProcessor;
    ///
    /// // Create weights for different sources
    /// let weights = HashMap::from([
    ///     ("binance".to_string(), 30),
    ///     ("coinbase".to_string(), 25),
    ///     ("kraken".to_string(), 20),
    ///     ("kucoin".to_string(), 15),
    /// ]);
    ///
    /// // Create a processor that requires a minimum cumulative weight of 50
    /// let processor = WeightedMedianProcessor::new(weights, 50);
    /// ```
    pub fn new(source_weights: HashMap<String, u32>, minimum_cumulative_weight: u32) -> Self {
        WeightedMedianProcessor {
            source_weights,
            minimum_cumulative_weight,
        }
    }
}

impl WeightedMedianProcessor {
    /// Calculates the weighted median of the provided values.
    ///
    /// This method computes the weighted median based on the source weights configuration.
    /// It first validates that all sources have defined weights and that the cumulative
    /// weight meets the minimum threshold.
    ///
    /// # Errors
    ///
    /// Returns a `ProcessError` if:
    /// - Any source in the input data does not have a defined weight
    /// - The cumulative weight of all provided sources is less than the minimum threshold
    pub fn process(&self, data: Vec<(String, Decimal)>) -> Result<Decimal, ProcessError> {
        let cumulative_weight = data.iter().try_fold(0, |acc, (source, _)| {
            self.source_weights
                .get(source)
                .map(|weight| acc + weight)
                .ok_or(ProcessError::new(format!("Unknown source {source}")))
        })?;

        if cumulative_weight < self.minimum_cumulative_weight {
            return Err(ProcessError::new(
                "Not enough sources to calculate weighted median",
            ));
        }

        let values = data
            .into_iter()
            .map(|(source, value)| {
                self.source_weights
                    .get(&source)
                    .map(|weight| (value, *weight))
                    .ok_or(ProcessError::new(format!("Unknown source {source}")))
            })
            .collect::<Result<Vec<(Decimal, u32)>, ProcessError>>()?;

        Ok(compute_weighted_median(values))
    }
}

/// Computes the weighted median of a collection of values with associated weights.
///
/// This function:
/// 1. Sorts values in ascending order
/// 2. Determines the midpoint based on the sum of all weights
/// 3. Traverses the sorted values, accumulating weights
/// 4. Returns the value where the accumulated weight crosses the midpoint
/// 5. If the midpoint falls exactly between two values, returns their average
///
/// The function assumes the input vector is non-empty and will panic otherwise.
fn compute_weighted_median<T>(mut values: Vec<(T, u32)>) -> T
where
    T: Ord + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    values.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));

    // We use the sum of the weights as the mid-value to find the median to avoid rounding when
    // dividing by two.
    let effective_mid = values
        .iter()
        .fold(u32::zero(), |acc, (_, weight)| acc + weight);

    let mut effective_cumulative_weight = u32::zero();
    let mut iter = values.into_iter();
    while let Some((value, weight)) = iter.next() {
        // We multiply the weight by 2 to avoid rounding when dividing by two.
        effective_cumulative_weight += weight * 2;
        match effective_cumulative_weight.cmp(&effective_mid) {
            Ordering::Greater => return value,
            Ordering::Equal => {
                return if let Some((next_value, _)) = iter.next() {
                    (value + next_value) / FromPrimitive::from_u32(2).unwrap()
                } else {
                    value
                };
            }
            Ordering::Less => (),
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_median() {
        let source_weights = HashMap::from([
            ("a".to_string(), 15),
            ("b".to_string(), 10),
            ("c".to_string(), 20),
            ("d".to_string(), 30),
            ("e".to_string(), 25),
        ]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, 0);
        let data = vec![
            ("a".to_string(), Decimal::from(1)),
            ("b".to_string(), Decimal::from(2)),
            ("c".to_string(), Decimal::from(3)),
            ("d".to_string(), Decimal::from(4)),
            ("e".to_string(), Decimal::from(5)),
        ];
        let res = weighted_median.process(data);

        assert_eq!(res.unwrap(), Decimal::from(4));
    }

    #[test]
    fn test_median_with_even_weight() {
        let source_weights = HashMap::from([("a".to_string(), 2)]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, 0);
        let data = vec![
            ("a".to_string(), Decimal::from(1)),
            ("a".to_string(), Decimal::from(2)),
            ("a".to_string(), Decimal::from(3)),
            ("a".to_string(), Decimal::from(4)),
            ("a".to_string(), Decimal::from(5)),
        ];
        let res = weighted_median.process(data);

        assert_eq!(res.unwrap(), Decimal::from(3));
    }

    #[test]
    fn test_weighted_median_with_intersect() {
        let source_weights = HashMap::from([
            ("a".to_string(), 49),
            ("b".to_string(), 1),
            ("c".to_string(), 25),
            ("d".to_string(), 25),
        ]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, 0);
        let data = vec![
            ("a".to_string(), Decimal::from(1)),
            ("b".to_string(), Decimal::from(2)),
            ("c".to_string(), Decimal::from(3)),
            ("d".to_string(), Decimal::from(4)),
        ];
        let res = weighted_median.process(data);

        assert_eq!(res.unwrap(), Decimal::from_str_exact("2.5").unwrap());
    }
}
