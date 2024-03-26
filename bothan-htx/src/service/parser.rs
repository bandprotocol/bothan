use bothan_core::types::PriceData;

use crate::api::types::Ticker;

pub fn parse_ticker(ticker: &Ticker, timestamp: usize) -> PriceData {
    PriceData::new(
        ticker.symbol.to_string(),
        ticker.close.to_string(),
        timestamp as u64,
    )
}

#[cfg(test)]
mod test {
    use bothan_core::types::PriceData;

    use crate::api::rest::test::mock_ticker;
    use crate::service::parser::parse_ticker;

    #[test]
    fn test_parse_quote() {
        let ticker = mock_ticker();
        let timestamp: usize = 100000;
        let result = parse_ticker(&ticker, timestamp);

        let expected = PriceData::new("btcusdt".to_string(), "80000".to_string(), 100000);
        assert_eq!(result, expected);
    }
}
