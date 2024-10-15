use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Div};

use bincode::{Decode, Encode};
use num_traits::{FromPrimitive, Zero};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::processor::{Process, ProcessError};

/// The `WeightedMedianProcessor` finds the weighted median of a given data set where the dataset
/// contains the source and the value. It also has a `minimum_cumulative_weight` which is the
/// minimum cumulative weight required to calculate the weighted median. If the cumulative weight
/// of the data sources is less than `minimum_cumulative_weight` or the source associated with the
/// data does not have an assigned weight, it returns an error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct WeightedMedianProcessor {
    pub source_weights: HashMap<String, u32>,
    pub minimum_cumulative_weight: u32,
}

impl WeightedMedianProcessor {
    /// Creates a new `WeightedMedianProcessor`.
    pub fn new(source_weights: HashMap<String, u32>, minimum_cumulative_weight: u32) -> Self {
        WeightedMedianProcessor {
            source_weights,
            minimum_cumulative_weight,
        }
    }
}

impl Process<(String, Decimal), Decimal> for WeightedMedianProcessor {
    /// Processes the given data and returns the weighted median. If the cumulative weights of the
    /// data sources are less than the minimum cumulative weight or the source associated
    /// with the data does not have an assigned weight, it returns an error.
    fn process(&self, data: Vec<(String, Decimal)>) -> Result<Decimal, ProcessError> {
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

// This function requires that values passed is not an empty vector, if an empty vector is passed,
// it will panic
fn compute_weighted_median<T>(mut values: Vec<(T, u32)>) -> T
where
    T: Ord + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    values.sort_by(|(v1, _), (v2, _)| v1.cmp(v2));

    let mid = values
        .iter()
        .fold(u32::zero(), |acc, (_, weight)| acc + weight);

    let mut cumulative_weight = u32::zero();
    let mut iter = values.into_iter();
    while let Some((value, weight)) = iter.next() {
        cumulative_weight += weight * 2;
        match cumulative_weight.cmp(&mid) {
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
