mod base;
mod pro;
mod public;

const USD: &str = "USD";
const PUBLIC_ENDPOINT: &str = "https://api.coingecko.com/api/v3/";
const PRO_ENDPOINT: &str = "https://pro-api.coingecko.com/api/v3/";

pub use pro::CoingeckoPro;
pub use public::CoingeckoPublic;
