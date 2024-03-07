use bothan_core::service::ServiceResult;
use bothan_core::types::PriceData as SourcePriceData;

use crate::proto::query::query::PriceData;

pub mod identity;
pub mod median;

pub trait Processor<E> {
    fn process(
        &self,
        data: Vec<ServiceResult<SourcePriceData>>,
        prerequisites: Vec<PriceData>,
    ) -> Result<f64, E>;
}
