use bincode::{Decode, Encode};
use num_traits::FromPrimitive;
use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessError;

const TICK: f64 = 1.0001;
const MID_TICK: f64 = 262144.0;
const MAX_TICK: f64 = 524287.0;
const MIN_TICK: f64 = 1.0;

/// `TickPostProcessor` processes the given data into its tick value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct TickPostProcessor {}

impl TickPostProcessor {
    /// Processes the given data into its tick value and returns it. If the data is out of bounds,
    /// it returns an error.
    pub fn process(&self, data: Decimal) -> Result<Decimal, PostProcessError> {
        // Unwrap here is safe because the constants are hardcoded.
        let tick = Decimal::from_f64(TICK).unwrap();
        let min_tick = Decimal::from_f64(MIN_TICK).unwrap();
        let mid_tick = Decimal::from_f64(MID_TICK).unwrap();
        let max_tick = Decimal::from_f64(MAX_TICK).unwrap();

        let result = (data.log10() / tick.log10()) + mid_tick;
        if !(min_tick..=max_tick).contains(&result) {
            return Err(PostProcessError::new("Tick value out of bound"));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::post_processor::PostProcessor;
    use crate::registry::post_processor::tick::TickPostProcessor;

    #[test]
    fn test_process() {
        let tick_convertor = PostProcessor::TickConvertor(TickPostProcessor {});
        let result = tick_convertor.post_process(Decimal::from(20));
        assert_eq!(
            result.unwrap(),
            Decimal::from_str_exact("292102.82057671349939971087257").unwrap()
        );
    }
}
