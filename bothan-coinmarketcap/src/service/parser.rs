use chrono::NaiveDateTime;

use bothan_core::types::PriceData;

use crate::api::types::Quote;

#[derive(Debug, thiserror::Error)]
pub enum QuoteParserError {
    #[error("invalid timestamp")]
    InvalidTimestamp,

    #[error("invalid price")]
    InvalidPrice,
}

pub fn parse_quote(quote: &Quote) -> Result<PriceData, QuoteParserError> {
    let price = quote
        .price_quotes
        .usd
        .price
        .ok_or(QuoteParserError::InvalidPrice)?;
    let last_updated = quote.price_quotes.usd.last_updated.as_str();
    let naive_date_time = NaiveDateTime::parse_from_str(last_updated, "%Y-%m-%dT%H:%M:%S.%fZ")
        .map_err(|_| QuoteParserError::InvalidTimestamp)?;
    let timestamp = u64::try_from(naive_date_time.and_utc().timestamp())
        .map_err(|_| QuoteParserError::InvalidTimestamp)?;

    Ok(PriceData::new(
        quote.id.to_string(),
        price.to_string(),
        timestamp,
    ))
}

#[cfg(test)]
mod test {
    use bothan_core::types::PriceData;

    use crate::api::rest::test::mock_quote;
    use crate::api::types::{PriceQuote, PriceQuotes, Quote};
    use crate::service::parser::parse_quote;

    #[test]
    fn test_parse_quote() {
        let quote = mock_quote();
        let result = parse_quote(&quote);

        let expected = PriceData::new("1".to_string(), "80000".to_string(), 1710572115);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let market = Quote {
            id: 112312312,
            symbol: "APB".to_string(),
            slug: "".to_string(),
            name: "Applebees".to_string(),
            price_quotes: PriceQuotes {
                usd: PriceQuote {
                    price: None,
                    volume_24h: 0.0,
                    volume_change_24h: 0.0,
                    market_cap: 0.0,
                    market_cap_dominance: 0.0,
                    fully_diluted_market_cap: 0.0,
                    percent_change_1h: 0.0,
                    percent_change_24h: 0.0,
                    percent_change_7d: 0.0,
                    percent_change_30d: 0.0,
                    last_updated: "never".to_string(),
                },
            },
        };
        assert!(parse_quote(&market).is_err());
    }
}
