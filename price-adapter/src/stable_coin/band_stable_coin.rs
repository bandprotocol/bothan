use super::types::StableCoin;
use crate::error::Error;

// A struct representing a static mapper using a HashMap of String keys to Values.
pub struct BandStableCoin {}

impl BandStableCoin {
    // Constructor to create a new BandStaticMapper from a pre-existing mapping.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BandStableCoin {
    fn default() -> Self {
        Self::new()
    }
}

// Implementing the Mapper trait for BandStaticMapper.
impl StableCoin for BandStableCoin {
    // Retrieve the mapping as a reference, wrapped in a Result.
    fn get_price(&self, _symbol: String) -> Result<f64, Error> {
        Ok(1_f64)
    }
}
