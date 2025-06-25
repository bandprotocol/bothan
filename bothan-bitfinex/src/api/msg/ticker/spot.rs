use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, de};

/// Represents spot ticker data from the Bitfinex API.
///
/// `Ticker` struct contains fields matching those returned by the Bitfinex API
/// for spot ticker events. It serves as an interface for JSON deserialization
/// of spot trading market data, supporting both array and object-based responses.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Ticker {
    /// The symbol of the requested ticker data (e.g., "tBTCUSD").
    pub symbol: String,
    /// Price of last highest bid.
    pub bid: f64,
    /// Sum of the 25 highest bid sizes.
    pub bid_size: f64,
    /// Price of last lowest ask.
    pub ask: f64,
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
            Bid,
            BidSize,
            Ask,
            AskSize,
            DailyChange,
            DailyChangeRelative,
            LastPrice,
            Volume,
            High,
            Low,
        }

        struct SpotTickerVisitor {}
        impl<'de> Visitor<'de> for SpotTickerVisitor {
            type Value = Ticker;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("tuple with length 11")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Ticker, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let symbol = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bid = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let bid_size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let ask = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let ask_size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let daily_change = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let daily_change_relative = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let last_price = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(7, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(8, &self))?;
                let high = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(9, &self))?;
                let low = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(10, &self))?;

                let spot_ticker = Ticker {
                    symbol,
                    bid,
                    bid_size,
                    ask,
                    ask_size,
                    daily_change,
                    daily_change_relative,
                    last_price,
                    volume,
                    high,
                    low,
                };
                Ok(spot_ticker)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut symbol = None;
                let mut bid = None;
                let mut bid_size = None;
                let mut ask = None;
                let mut ask_size = None;
                let mut daily_change = None;
                let mut daily_change_relative = None;
                let mut last_price = None;
                let mut volume = None;
                let mut high = None;
                let mut low = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Symbol => {
                            if symbol.is_some() {
                                return Err(de::Error::duplicate_field("symbol"));
                            }
                            symbol = Some(map.next_value()?);
                        }
                        Field::Bid => {
                            if bid.is_some() {
                                return Err(de::Error::duplicate_field("bid"));
                            }
                            bid = Some(map.next_value()?);
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
                    }
                }

                let symbol = symbol.ok_or_else(|| de::Error::missing_field("symbol"))?;
                let bid = bid.ok_or_else(|| de::Error::missing_field("bid"))?;
                let bid_size = bid_size.ok_or_else(|| de::Error::missing_field("bid_size"))?;
                let ask = ask.ok_or_else(|| de::Error::missing_field("ask"))?;
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

                let ticker = Ticker {
                    symbol,
                    bid,
                    bid_size,
                    ask,
                    ask_size,
                    daily_change,
                    daily_change_relative,
                    last_price,
                    volume,
                    high,
                    low,
                };
                Ok(ticker)
            }
        }

        const FIELDS: &[&str] = &[
            "symbol",
            "bid",
            "bid_size",
            "ask",
            "ask_size",
            "daily_change",
            "daily_change_relative",
            "last_price",
            "volume",
            "high",
            "low",
        ];
        deserializer.deserialize_struct("SpotTicker", FIELDS, SpotTickerVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_spot_ticker_from_array() {
        let json = r#"["tBTCUSD",101740,93.86022424,101750,38.06413103,2132,0.02140175,101750,663.27534767,102760,98740]"#;
        let spot_ticker: Ticker = serde_json::from_str(json).unwrap();

        let expected = Ticker {
            symbol: "tBTCUSD".to_string(),
            bid: 101740.0,
            bid_size: 93.86022424,
            ask: 101750.0,
            ask_size: 38.06413103,
            daily_change: 2132.0,
            daily_change_relative: 0.02140175,
            last_price: 101750.0,
            volume: 663.27534767,
            high: 102760.0,
            low: 98740.0,
        };

        assert_eq!(spot_ticker, expected);
    }

    #[test]
    fn test_deserialize_spot_ticker_from_array_with_invalid_length() {
        let json = r#"["tBTCUSD",101740,93.86022424,101750,38.06413103,2132,0.02140175,101750,663.27534767]"#;
        let spot_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            spot_ticker.err().unwrap().to_string(),
            "invalid length 9, expected tuple with length 11 at line 1 column 85"
        );
    }

    #[test]
    fn test_deserialize_spot_ticker_from_array_with_invalid_value() {
        let json =
            r#"[100000,101740,93.86022424,101750,38.06413103,2132,0.02140175,101750,663.27534767]"#;
        let spot_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            spot_ticker.err().unwrap().to_string(),
            "invalid type: integer `100000`, expected a string at line 1 column 7"
        );
    }

    #[test]
    fn test_deserialize_spot_ticker_from_map() {
        let json = r#"{"symbol":"tBTCUSD","bid":101740,"bid_size":93.86022424,"ask":101750,"ask_size":38.06413103,"daily_change":2132,"daily_change_relative":0.02140175,"last_price":101750,"volume":663.27534767,"high":102760,"low":98740.0}"#;
        let spot_ticker: Ticker = serde_json::from_str(json).unwrap();

        let expected = Ticker {
            symbol: "tBTCUSD".to_string(),
            bid: 101740.0,
            bid_size: 93.86022424,
            ask: 101750.0,
            ask_size: 38.06413103,
            daily_change: 2132.0,
            daily_change_relative: 0.02140175,
            last_price: 101750.0,
            volume: 663.27534767,
            high: 102760.0,
            low: 98740.0,
        };

        assert_eq!(spot_ticker, expected);
    }

    #[test]
    fn test_deserialize_spot_ticker_from_map_with_missing_field() {
        let json = r#"{"bid":101740,"bid_size":93.86022424,"ask":101750,"ask_size":38.06413103,"daily_change":2132,"daily_change_relative":0.02140175,"last_price":101750,"volume":663.27534767,"high":102760,"low":98740.0}"#;
        let spot_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            spot_ticker.err().unwrap().to_string(),
            "missing field `symbol` at line 1 column 198"
        );
    }

    #[test]
    fn test_deserialize_spot_ticker_from_map_with_invalid_key() {
        let json = r#"{"abc":"tBTCUSD","bid":101740,"bid_size":93.86022424,"ask":101750,"ask_size":38.06413103,"daily_change":2132,"daily_change_relative":0.02140175,"last_price":101750,"volume":663.27534767,"high":102760,"low":98740.0}"#;
        let spot_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            spot_ticker.err().unwrap().to_string(),
            "unknown field `abc`, expected one of `symbol`, `bid`, `bid_size`, `ask`, `ask_size`, `daily_change`, `daily_change_relative`, `last_price`, `volume`, `high`, `low` at line 1 column 6"
        );
    }

    #[test]
    fn test_deserialize_spot_ticker_from_map_with_invalid_value() {
        let json = r#"{"symbol": -1000,"bid":101740,"bid_size":93.86022424,"ask":101750,"ask_size":38.06413103,"daily_change":2132,"daily_change_relative":0.02140175,"last_price":101750,"volume":663.27534767,"high":102760,"low":98740.0}"#;
        let spot_ticker: Result<Ticker, _> = serde_json::from_str(json);

        assert_eq!(
            spot_ticker.err().unwrap().to_string(),
            "invalid type: integer `-1000`, expected a string at line 1 column 16"
        );
    }
}
