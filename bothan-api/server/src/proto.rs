#[allow(clippy::all)]
#[rustfmt::skip]
pub mod query;

impl query::Price {
    pub fn new<T: Into<String>, U: Into<i64>>(
        signal_id: T,
        price: U,
        status: query::PriceStatus,
    ) -> query::Price {
        query::Price {
            signal_id: signal_id.into(),
            price: price.into(),
            status: status.into(),
        }
    }
}
