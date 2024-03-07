use num_traits::Float;

use crate::post_processors::PostProcessor;

const TICK: f64 = 1.0001;
const MID_TICK: f64 = 262144.0;
const MAX_TICK: f64 = 524287.0;
const MIN_TICK: f64 = 1.0;

pub struct TickPostProcessor {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("price out of bound")]
    OutOfBound,
}

impl PostProcessor<Error> for TickPostProcessor {
    fn process(&self, data: f64) -> Result<f64, Error> {
        let tick = to_tick(data).ok_or(Error::OutOfBound)?;
        Ok(tick)
    }
}

fn to_tick(price: f64) -> Option<f64> {
    let tick = (price.log10() / TICK.log10()) + MID_TICK;
    if tick < MIN_TICK || tick > MAX_TICK {
        return None;
    }

    Some(tick)
}
