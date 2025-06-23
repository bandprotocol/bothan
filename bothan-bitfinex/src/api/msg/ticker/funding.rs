use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, de};

/// Represents funding ticker data from the Bitfinex API.
///
/// `Ticker` struct contains fields matching those returned by the Bitfinex API
/// for funding ticker events. It serves as an interface for JSON deserialization
/// of funding market data, supporting both array and object-based responses.
///
/// # Examples
///
/// ```rust
/// use bothan_bitfinex::api::msg::ticker::funding::Ticker;
/// use serde_json::json;
///
/// let json_data = json!(["fUSD",0.00018055342465753425,0.0002,120,35545399.51575242,0.00008219178082191781,2,28117235.06098758,-0.0000278,-0.2528,0.00008219,413386933.358769,0.000137,0.000025,null,null,5817583.43063814]);
///
/// let funding_ticker: Ticker = serde_json::from_value(json_data).unwrap();
///
/// assert_eq!(funding_ticker.symbol, "fUSD");
/// assert_eq!(funding_ticker.frr, 0.00018055342465753425);
/// assert_eq!(funding_ticker.bid, 0.0002);
/// assert_eq!(funding_ticker.bid_period, 120);
/// assert_eq!(funding_ticker.last_price, 0.00008219);
/// ```
///
/// # Bitfinex API Response Example
///
/// ```json
/// ["fUSD",0.00018055342465753425,0.0002,120,35545399.51575242,0.00008219178082191781,2,28117235.06098758,-0.0000278,-0.2528,0.00008219,413386933.358769,0.000137,0.000025,null,null,5817583.43063814]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Ticker {
    /// The symbol of the requested ticker data (e.g., "fUSD").
    pub symbol: String,
    /// Flash Return Rate - average of all fixed rate funding over the last hour.
    pub frr: f64,
    /// Price of last highest bid.
    pub bid: f64,
    /// Bid period covered (in days).
    pub bid_period: i64,
    /// Sum of the 25 highest bid sizes.
    pub bid_size: f64,
    /// Price of last lowest ask.
    pub ask: f64,
    /// Ask period covered (in days).
    pub ask_period: i64,
    /// Sum of the 25 lowest ask sizes.
    pub ask_size: f64,
    /// The amount that the last price has changed since yesterday.
    pub daily_change: f64,
    /// Relative price change since yesterday (*100 for percentage change).
    pub daily_change_relative: f64,
    /// Price of the last trade.
    pub last_price: f64,
    /// Daily volume.
    pub volume: f64,
    /// Daily high.
    pub high: f64,
    /// Daily low.
    pub low: f64,
    /// The amount of funding that is available at the Flash Return Rate.
    pub frr_amount_available: f64,
}

impl<'de> Deserialize<'de> for Ticker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Symbol,
            Frr,
            Bid,
            BidPeriod,
            BidSize,
            Ask,
            AskPeriod,
            AskSize,
            DailyChange,
            DailyChangeRelative,
            LastPrice,
            Volume,
            High,
            Low,
            FrrAmountAvailable,
        }

        struct FundingTickerVisitor {}
        impl<'de> Visitor<'de> for FundingTickerVisitor {
            type Value = Ticker;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("tuple with length 15")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Ticker, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let symbol = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let frr = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let bid = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let bid_period = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let bid_size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let ask = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let ask_period = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let ask_size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(7, &self))?;
                let daily_change = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(8, &self))?;
                let daily_change_relative = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(9, &self))?;
                let last_price = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(10, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(11, &self))?;
                let high = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(12, &self))?;
                let low = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(13, &self))?;
                // Skip the next two elements as they're reserved and currently only contain nil
                let _ = seq
                    .next_element::<Option<String>>()?
                    .ok_or_else(|| de::Error::invalid_length(14, &self))?;
                let _ = seq
                    .next_element::<Option<String>>()?
                    .ok_or_else(|| de::Error::invalid_length(15, &self))?;
                let frr_amount_available = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(16, &self))?;

                let funding_ticker = Ticker {
                    symbol,
                    frr,
                    bid,
                    bid_period,
                    bid_size,
                    ask,
                    ask_period,
                    ask_size,
                    daily_change,
                    daily_change_relative,
                    last_price,
                    volume,
                    high,
                    low,
                    frr_amount_available,
                };
                Ok(funding_ticker)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut symbol = None;
                let mut frr = None;
                let mut bid = None;
                let mut bid_period = None;
                let mut bid_size = None;
                let mut ask = None;
                let mut ask_period = None;
                let mut ask_size = None;
                let mut daily_change = None;
                let mut daily_change_relative = None;
                let mut last_price = None;
                let mut volume = None;
                let mut high = None;
                let mut low = None;
                let mut frr_amount_available = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Symbol => {
                            if symbol.is_some() {
                                return Err(de::Error::duplicate_field("symbol"));
                            }
                            symbol = Some(map.next_value()?);
                        }
                        Field::Frr => {
                            if frr.is_some() {
                                return Err(de::Error::duplicate_field("frr"));
                            }
                            frr = Some(map.next_value()?);
                        }
                        Field::Bid => {
                            if bid.is_some() {
                                return Err(de::Error::duplicate_field("bid"));
                            }
                            bid = Some(map.next_value()?);
                        }
                        Field::BidPeriod => {
                            if bid_period.is_some() {
                                return Err(de::Error::duplicate_field("bid_period"));
                            }
                            bid_period = Some(map.next_value()?);
                        }
                        Field::BidSize => {
                            if bid_size.is_some() {
                                return Err(de::Error::duplicate_field("bid_size"));
                            }
                            bid_size = Some(map.next_value()?);
                        }
                        Field::Ask => {
                            if ask.is_some() {
                                return Err(de::Error::duplicate_field("ask"));
                            }
                            ask = Some(map.next_value()?);
                        }
                        Field::AskPeriod => {
                            if ask_period.is_some() {
                                return Err(de::Error::duplicate_field("ask_period"));
                            }
                            ask_period = Some(map.next_value()?);
                        }
                        Field::AskSize => {
                            if ask_size.is_some() {
                                return Err(de::Error::duplicate_field("ask_size"));
                            }
                            ask_size = Some(map.next_value()?);
                        }
                        Field::DailyChange => {
                            if daily_change.is_some() {
                                return Err(de::Error::duplicate_field("daily_change"));
                            }
                            daily_change = Some(map.next_value()?);
                        }
                        Field::DailyChangeRelative => {
                            if daily_change_relative.is_some() {
                                return Err(de::Error::duplicate_field("daily_change_relative"));
                            }
                            daily_change_relative = Some(map.next_value()?);
                        }
                        Field::LastPrice => {
                            if last_price.is_some() {
                                return Err(de::Error::duplicate_field("last_price"));
                            }
                            last_price = Some(map.next_value()?);
                        }
                        Field::Volume => {
                            if volume.is_some() {
                                return Err(de::Error::duplicate_field("volume"));
                            }
                            volume = Some(map.next_value()?);
                        }
                        Field::High => {
                            if high.is_some() {
                                return Err(de::Error::duplicate_field("high"));
                            }
                            high = Some(map.next_value()?);
                        }
                        Field::Low => {
                            if low.is_some() {
                                return Err(de::Error::duplicate_field("low"));
                            }
                            low = Some(map.next_value()?);
                        }
                        Field::FrrAmountAvailable => {
                            if frr_amount_available.is_some() {
                                return Err(de::Error::duplicate_field("frr_amount_available"));
                            }
                            frr_amount_available = Some(map.next_value()?);
                        }
                    }
                }

                let symbol = symbol.ok_or_else(|| de::Error::missing_field("symbol"))?;
                let frr = frr.ok_or_else(|| de::Error::missing_field("frr"))?;
                let bid = bid.ok_or_else(|| de::Error::missing_field("bid"))?;
                let bid_period =
                    bid_period.ok_or_else(|| de::Error::missing_field("bid_period"))?;
                let bid_size = bid_size.ok_or_else(|| de::Error::missing_field("bid_size"))?;
                let ask = ask.ok_or_else(|| de::Error::missing_field("ask"))?;
                let ask_period =
                    ask_period.ok_or_else(|| de::Error::missing_field("ask_period"))?;
                let ask_size = ask_size.ok_or_else(|| de::Error::missing_field("ask_size"))?;
                let daily_change =
                    daily_change.ok_or_else(|| de::Error::missing_field("daily_change"))?;
                let daily_change_relative = daily_change_relative
                    .ok_or_else(|| de::Error::missing_field("daily_change_relative"))?;
                let last_price =
                    last_price.ok_or_else(|| de::Error::missing_field("last_price"))?;
                let volume = volume.ok_or_else(|| de::Error::missing_field("volume"))?;
                let high = high.ok_or_else(|| de::Error::missing_field("high"))?;
                let low = low.ok_or_else(|| de::Error::missing_field("low"))?;
                let frr_amount_available = frr_amount_available
                    .ok_or_else(|| de::Error::missing_field("frr_amount_available"))?;

                let ticker = Ticker {
                    symbol,
                    frr,
                    bid,
                    bid_period,
                    bid_size,
                    ask,
                    ask_period,
                    ask_size,
                    daily_change,
                    daily_change_relative,
                    last_price,
                    volume,
                    high,
                    low,
                    frr_amount_available,
                };

                Ok(ticker)
            }
        }

        const FIELDS: &[&str] = &[
            "symbol",
            "frr",
            "bid",
            "bid_period",
            "bid_size",
            "ask",
            "ask_period",
            "ask_size",
            "daily_change",
            "daily_change_relative",
            "last_price",
            "volume",
            "high",
            "low",
            "frr_amount_available",
        ];
        deserializer.deserialize_struct("FundingTicker", FIELDS, FundingTickerVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_funding_ticker_from_array() {
        let json = r#"["fUSD",0.00018055342465753425,0.0002,120,35545399.51575242,0.00008219178082191781,2,28117235.06098758,-0.0000278,-0.2528,0.00008219,413386933.358769,0.000137,0.000025,null,null,5817583.43063814]"#;
        let funding_ticker: Ticker = serde_json::from_str(json).unwrap();

        let expected = Ticker {
            symbol: "fUSD".to_string(),
            frr: 0.00018055342465753425,
            bid: 0.0002,
            bid_period: 120,
            bid_size: 35545399.51575242,
            ask: 0.00008219178082191781,
            ask_period: 2,
            ask_size: 28117235.06098758,
            daily_change: -0.0000278,
            daily_change_relative: -0.2528,
            last_price: 0.00008219,
            volume: 413386933.358769,
            high: 0.000137,
            low: 0.000025,
            frr_amount_available: 5817583.43063814,
        };

        assert_eq!(funding_ticker, expected);
    }

    #[test]
    fn test_deserialize_funding_ticker_from_array_with_invalid_length() {
        let json = r#"["fUSD",0.00018055342465753425,0.0002,120,35545399.51575242,0.00008219178082191781,2,28117235.06098758,-0.0000278,-0.2528,0.00008219,413386933.358769,0.000137,0.000025,null,null]"#;
        let funding_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            funding_ticker.err().unwrap().to_string(),
            "invalid length 16, expected tuple with length 15 at line 1 column 178"
        );
    }

    #[test]
    fn test_deserialize_funding_ticker_from_array_with_invalid_value() {
        let json = r#"[1000000,0.00018055342465753425,0.0002,120,35545399.51575242,0.00008219178082191781,2,28117235.06098758,-0.0000278,-0.2528,0.00008219,413386933.358769,0.000137,0.000025,null,null,5817583.43063814]"#;
        let funding_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            funding_ticker.err().unwrap().to_string(),
            "invalid type: integer `1000000`, expected a string at line 1 column 8"
        );
    }

    #[test]
    fn test_deserialize_funding_ticker_from_map() {
        let json = r#"{"symbol":"fUSD","frr":0.00018055342465753425,"bid":0.0002,"bid_period":120,"bid_size":35545399.51575242,"ask":0.00008219178082191781,"ask_period":2,"ask_size":28117235.06098758,"daily_change":-0.0000278,"daily_change_relative":-0.2528,"last_price":0.00008219,"volume":413386933.358769,"high":0.000137,"low":0.000025,"frr_amount_available":5817583.43063814}"#;
        let funding_ticker: Ticker = serde_json::from_str(json).unwrap();

        let expected = Ticker {
            symbol: "fUSD".to_string(),
            frr: 0.00018055342465753425,
            bid: 0.0002,
            bid_period: 120,
            bid_size: 35545399.51575242,
            ask: 0.00008219178082191781,
            ask_period: 2,
            ask_size: 28117235.06098758,
            daily_change: -0.0000278,
            daily_change_relative: -0.2528,
            last_price: 0.00008219,
            volume: 413386933.358769,
            high: 0.000137,
            low: 0.000025,
            frr_amount_available: 5817583.43063814,
        };

        assert_eq!(funding_ticker, expected);
    }

    #[test]
    fn test_deserialize_funding_ticker_from_map_with_missing_field() {
        let json = r#"{"symbol":"fUSD","frr":0.00018055342465753425,"bid":0.0002,"bid_period":120,"bid_size":35545399.51575242,"ask":0.00008219178082191781,"ask_period":2,"ask_size":28117235.06098758,"daily_change":-0.0000278,"daily_change_relative":-0.2528,"last_price":0.00008219,"volume":413386933.358769,"high":0.000137,"low":0.000025}"#;
        let funding_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            funding_ticker.err().unwrap().to_string(),
            "missing field `frr_amount_available` at line 1 column 317"
        );
    }

    #[test]
    fn test_deserialize_funding_ticker_from_map_with_invalid_key() {
        let json = r#"{"abc":"fUSD","frr":0.00018055342465753425,"bid":0.0002,"bid_period":120,"bid_size":35545399.51575242,"ask":0.00008219178082191781,"ask_period":2,"ask_size":28117235.06098758,"daily_change":-0.0000278,"daily_change_relative":-0.2528,"last_price":0.00008219,"volume":413386933.358769,"high":0.000137,"low":0.000025,"frr_amount_available":5817583.43063814}"#;
        let funding_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            funding_ticker.err().unwrap().to_string(),
            "unknown field `abc`, expected one of `symbol`, `frr`, `bid`, `bid_period`, `bid_size`, `ask`, `ask_period`, `ask_size`, `daily_change`, `daily_change_relative`, `last_price`, `volume`, `high`, `low`, `frr_amount_available` at line 1 column 6"
        );
    }

    #[test]
    fn test_deserialize_funding_ticker_from_map_with_invalid_value() {
        let json = r#"{"symbol": 0,"frr":0.00018055342465753425,"bid":0.0002,"bid_period":120,"bid_size":35545399.51575242,"ask":0.00008219178082191781,"ask_period":2,"ask_size":28117235.06098758,"daily_change":-0.0000278,"daily_change_relative":-0.2528,"last_price":0.00008219,"volume":413386933.358769,"high":0.000137,"low":0.000025,"frr_amount_available":5817583.43063814}"#;
        let funding_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            funding_ticker.err().unwrap().to_string(),
            "invalid type: integer `0`, expected a string at line 1 column 12"
        );
    }
}
