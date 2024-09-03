#[allow(clippy::all)]
#[rustfmt::skip]
pub mod price;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod signal;

impl price::Price {
    pub fn new<T: Into<String>, U: Into<i64>>(
        signal_id: T,
        price: U,
        status: price::Status,
    ) -> price::Price {
        price::Price {
            signal_id: signal_id.into(),
            price: price.into(),
            status: status.into(),
        }
    }
}
