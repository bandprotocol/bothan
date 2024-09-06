use bothan_core::types::AssetInfo;

use crate::api::types::Ticker;

/// Parses a `Ticker` into a `PriceData` object.
pub fn parse_ticker(ticker: &Ticker, timestamp: usize) -> AssetInfo {
    AssetInfo::new(
        ticker.symbol.to_string(),
        ticker.close.to_string(),
        timestamp as u64,
    )
}

#[cfg(test)]
mod test {
    use bothan_core::types::AssetInfo;

    use crate::api::rest::test::mock_ticker;
    use crate::service::parser::parse_ticker;

    #[test]
    fn test_parse_quote() {
        let ticker = mock_ticker();
        let timestamp: usize = 100000;
        let result = parse_ticker(&ticker, timestamp);

        let expected = AssetInfo::new("btcusdt".to_string(), "80000".to_string(), 100000);
        assert_eq!(result, expected);
    }
}
