use crate::error::Error;
use crate::types::StableCoin;

/// A struct representing a stable coin with a constant price.
pub struct BandStableCoin;

impl BandStableCoin {
    /// Constructor to create a new BandStableCoin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for BandStableCoin {
    fn default() -> Self {
        Self::new()
    }
}

// Implementing the StableCoin trait for BandStableCoin.
impl StableCoin for BandStableCoin {
    /// Retrieve the price of the stable coin.
    ///
    /// This method returns a constant value of 1.0 for any symbol.
    fn get_price(&self, _symbol: String) -> Result<f64, Error> {
        Ok(1.0)
    }
}
