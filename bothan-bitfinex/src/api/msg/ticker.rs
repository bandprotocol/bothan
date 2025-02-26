mod funding;
mod spot;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ticker {
    Funding(funding::Ticker),
    Spot(spot::Ticker),
}

impl Ticker {
    pub fn symbol(&self) -> &str {
        match self {
            Ticker::Funding(t) => &t.symbol,
            Ticker::Spot(t) => &t.symbol,
        }
    }

    pub fn price(&self) -> f64 {
        match self {
            Ticker::Funding(t) => t.last_price,
            Ticker::Spot(t) => t.last_price,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_tickers_from_array() {
        let json = r#"[["tBTCUSD",101530,39.76548266,101540,32.24226311,2680,0.0271063,101550,661.88869229,102760,98740],["fUSD",0.000180427397260274,0.0002,120,35441993.51575242,0.00008219,2,39208.22419296,-0.00005519,-0.5017,0.00005481,406448929.8255126,0.000137,0.000024,null,null,5863426.35928275]]"#;
        let ticker: Vec<Ticker> = serde_json::from_str(json).unwrap();

        let expected = vec![
            Ticker::Spot(spot::Ticker {
                symbol: "tBTCUSD".to_string(),
                bid: 101530.0,
                bid_size: 39.76548266,
                ask: 101540.0,
                ask_size: 32.24226311,
                daily_change: 2680.0,
                daily_change_relative: 0.0271063,
                last_price: 101550.0,
                volume: 661.88869229,
                high: 102760.0,
                low: 98740.0,
            }),
            Ticker::Funding(funding::Ticker {
                symbol: "fUSD".to_string(),
                frr: 0.000180427397260274,
                bid: 0.0002,
                bid_period: 120,
                bid_size: 35441993.51575242,
                ask: 0.00008219,
                ask_period: 2,
                ask_size: 39208.22419296,
                daily_change: -0.00005519,
                daily_change_relative: -0.5017,
                last_price: 0.00005481,
                volume: 406448929.8255126,
                high: 0.000137,
                low: 0.000024,
                frr_amount_available: 5863426.35928275,
            }),
        ];
        assert_eq!(ticker, expected);
    }
}
