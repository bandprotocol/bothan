use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Div};

use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use num_traits::{FromPrimitive, Zero};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::registry::processor::{Process, ProcessError};

/// The `WeightedMedianProcessor` finds the weighted median of a given data set where the dataset
/// contains the source and the value. It also has a `minimum_cumulative_weight` which is the
/// minimum cumulative weight required to calculate the weighted median. If the cumulative weight
/// of the data sources is less than `minimum_cumulative_weight` or the source associated with the
/// data does not have an assigned weight, it returns an error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeightedMedianProcessor {
    pub source_weights: HashMap<String, Decimal>,
    pub minimum_cumulative_weight: Decimal,
}

impl Encode for WeightedMedianProcessor {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let encodable_map = self
            .source_weights
            .iter()
            .map(|(k, v)| (k, v.serialize()))
            .collect::<HashMap<&String, [u8; 16]>>();
        Encode::encode(&encodable_map, encoder)?;
        Encode::encode(&self.minimum_cumulative_weight.serialize(), encoder)
    }
}

impl Decode for WeightedMedianProcessor {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let serialized_decimal_map: HashMap<String, [u8; 16]> = Decode::decode(decoder)?;
        let source_weights = serialized_decimal_map
            .into_iter()
            .map(|(k, v)| (k, Decimal::deserialize(v)))
            .collect();

        let minimum_cumulative_weight = Decimal::deserialize(Decode::decode(decoder)?);

        Ok(WeightedMedianProcessor {
            source_weights,
            minimum_cumulative_weight,
        })
    }
}

impl<'de> BorrowDecode<'de> for WeightedMedianProcessor {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let serialized_decimal_map: HashMap<String, [u8; 16]> =
            BorrowDecode::borrow_decode(decoder)?;
        let source_weights = serialized_decimal_map
            .into_iter()
            .map(|(k, v)| (k, Decimal::deserialize(v)))
            .collect();

        let minimum_cumulative_weight = Decimal::deserialize(BorrowDecode::borrow_decode(decoder)?);

        Ok(WeightedMedianProcessor {
            source_weights,
            minimum_cumulative_weight,
        })
    }
}

impl WeightedMedianProcessor {
    /// Creates a new `WeightedMedianProcessor`.
    pub fn new(
        source_weights: HashMap<String, Decimal>,
        minimum_cumulative_weight: Decimal,
    ) -> Self {
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
        let cumulative_weight = data.iter().try_fold(Decimal::zero(), |acc, (source, _)| {
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
            .collect::<Result<Vec<(Decimal, Decimal)>, ProcessError>>()?;

        let median = compute_weighted_median(values)
            .ok_or_else(|| ProcessError::new("Failed to calculate weighted median"))?;
        Ok(median)
    }
}

fn compute_weighted_median<T>(mut values: Vec<(T, Decimal)>) -> Option<T>
where
    T: Ord + Copy + Add<Output = T> + Div<Output = T> + FromPrimitive,
{
    values.sort_by(|a, b| a.0.cmp(&b.0));

    let sum = values
        .iter()
        .fold(Decimal::zero(), |acc, (_, weight)| acc + weight);

    let mid = sum / Decimal::new(2, 0);

    let mut cumulative_weight = Decimal::zero();
    let mut iter = values.into_iter();
    while let Some((value, weight)) = iter.next() {
        cumulative_weight += weight;
        match cumulative_weight.cmp(&mid) {
            Ordering::Greater => return Some(value),
            Ordering::Equal => {
                return if let Some((next_value, _)) = iter.next() {
                    Some((value + next_value) / FromPrimitive::from_u32(2).unwrap())
                } else {
                    Some(value)
                };
            }
            Ordering::Less => (),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_median() {
        let source_weights = HashMap::from([
            ("a".to_string(), Decimal::from_str_exact("0.15").unwrap()),
            ("b".to_string(), Decimal::from_str_exact("0.1").unwrap()),
            ("c".to_string(), Decimal::from_str_exact("0.2").unwrap()),
            ("d".to_string(), Decimal::from_str_exact("0.3").unwrap()),
            ("e".to_string(), Decimal::from_str_exact("0.25").unwrap()),
        ]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, Decimal::zero());
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
        let source_weights =
            HashMap::from([("a".to_string(), Decimal::from_str_exact("0.2").unwrap())]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, Decimal::zero());
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
            ("a".to_string(), Decimal::from_str_exact("0.49").unwrap()),
            ("b".to_string(), Decimal::from_str_exact("0.01").unwrap()),
            ("c".to_string(), Decimal::from_str_exact("0.25").unwrap()),
            ("d".to_string(), Decimal::from_str_exact("0.25").unwrap()),
        ]);

        let weighted_median = WeightedMedianProcessor::new(source_weights, Decimal::zero());
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
