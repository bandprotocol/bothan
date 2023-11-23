use crate::error::Error;

/// Trait for interacting with stable coins and retrieving their prices.
///
/// This trait defines a method for obtaining the price of a stable coin for a given symbol.
pub trait StableCoin: Send + Sync + Sized + Unpin + 'static {
    /// Gets the price of the stable coin for the specified symbol.
    ///
    /// This method takes a symbol representing a stable coin and returns a `Result<f64, Error>`,
    /// where the `f64` is the price of the stable coin and `Error` represents any error that
    /// may occur during the operation.
    fn get_price(&self, symbol: String) -> Result<f64, Error>;
}
