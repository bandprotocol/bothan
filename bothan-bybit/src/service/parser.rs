use bothan_core::types::PriceData;

use crate::api::types::ticker::{SpotTicker, Tickers};

/// Parses a `SpotTicker` into a `PriceData` object.
pub fn parse_spot_ticker(ticker: &SpotTicker, timestamp: usize) -> PriceData {
    PriceData::new(
        ticker.symbol.clone(),
        ticker.last_price.clone(),
        timestamp as u64,
    )
}

/// Parses a `Tickers` object into a vector of `PriceData` objects.
pub fn parse_tickers(tickers: &Tickers, timestamp: usize) -> Vec<PriceData> {
    match &tickers {
        Tickers::Spot(spot_tickers) => spot_tickers
            .iter()
            .map(|ticker| parse_spot_ticker(ticker, timestamp))
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use bothan_core::types::PriceData;

    use crate::api::types::ticker::SpotTicker;
    use crate::service::parser::parse_spot_ticker;

    #[test]
    fn test_parse_quote() {
        let spot_ticker = SpotTicker {
            symbol: "BTCUSDT".to_string(),
            bid1_price: "80000".to_string(),
            bid1_size: "1000000".to_string(),
            ask1_price: "80000".to_string(),
            ask1_size: "1000000".to_string(),
            last_price: "80000".to_string(),
            prev_price_24h: "80000".to_string(),
            price_24h_pcnt: "0".to_string(),
            high_price_24h: "80000".to_string(),
            low_price_24h: "80000".to_string(),
            turnover_24h: "0".to_string(),
            volume_24h: "800000".to_string(),
        };

        let timestamp: usize = 100000;
        let result = parse_spot_ticker(&spot_ticker, timestamp);

        let expected = PriceData::new("BTCUSDT".to_string(), "80000".to_string(), 100000);
        assert_eq!(result, expected);
    }
}
