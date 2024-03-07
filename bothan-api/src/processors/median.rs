use std::cmp::Ordering;
use std::str::FromStr;

use num_traits::{Float, NumCast};

use bothan_core::service::ServiceResult;
use bothan_core::types::PriceData as SourcePriceData;

use crate::processors::Processor;
use crate::proto::query::query::PriceData;

pub struct MedianProcessor {
    min_source_count: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not enough valid sources")]
    NotEnoughSources,

    #[error("invalid price")]
    InvalidPrice(#[from] std::num::ParseFloatError),
}

impl Processor<Error> for MedianProcessor {
    fn process(
        &self,
        data: Vec<ServiceResult<SourcePriceData>>,
        _: Vec<PriceData>,
    ) -> Result<f64, Error> {
        let prices = data
            .into_iter()
            .filter_map(|r| {
                r.ok()
                    .map(|pd| pd.price)
                    .and_then(|price| f64::from_str(price.as_str()).ok())
            })
            .collect::<Vec<f64>>();

        if prices.len() < self.min_source_count {
            return Err(Error::NotEnoughSources);
        }

        Ok(median(prices))
    }
}

pub fn median<T: Float>(mut data: Vec<T>) -> T {
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
