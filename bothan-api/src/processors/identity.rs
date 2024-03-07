use crate::processors::Processor;
use crate::proto::query::query::PriceData;
use bothan_core::service::ServiceResult;
use bothan_core::types::PriceData as SourcePriceData;
use std::str::FromStr;

pub struct IdentityProcessor {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid prerequisites amount")]
    InvalidPrerequisitesAmount,

    #[error("invalid price")]
    InvalidPrice(#[from] std::num::ParseFloatError),
}

impl Processor<Error> for IdentityProcessor {
    fn process(
        &self,
        _: Vec<ServiceResult<SourcePriceData>>,
        prerequisites: Vec<PriceData>,
    ) -> Result<f64, Error> {
        if prerequisites.len() != 1 {
            return Err(Error::InvalidPrerequisitesAmount);
        }

        Ok(f64::from_str(prerequisites[0].price.as_str())?)
    }
}
