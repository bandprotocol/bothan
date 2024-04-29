use serde::{Deserialize, Serialize};

use crate::post_processor::{PostProcessor, PostProcessorError};

const TICK: f64 = 1.0001;
const MID_TICK: f64 = 262144.0;
const MAX_TICK: f64 = 524287.0;
const MIN_TICK: f64 = 1.0;

/// `TickPostProcessor` processes the given data into its tick value.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TickPostProcessor {}

impl PostProcessor for TickPostProcessor {
    /// Processes the given data into its tick value and returns it. If the data is out of bounds,
    /// it returns an error.
    fn process(&self, data: f64) -> Result<f64, PostProcessorError> {
        let tick = (data.log10() / TICK.log10()) + MID_TICK;
        if !(MIN_TICK..=MAX_TICK).contains(&tick) {
            return Err(PostProcessorError::OutOfBound);
        }

        Ok(tick)
    }
}
