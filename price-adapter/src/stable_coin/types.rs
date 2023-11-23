use crate::error::Error;

pub trait StableCoin: Send + Sync + Sized + Unpin + 'static {
    fn get_price(&self, symbol: String) -> Result<f64, Error>;
}
