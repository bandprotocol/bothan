use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use rust_decimal::Decimal;

use crate::manager::crypto_asset_info::types::PriceState;

pub struct PriceCache<K> {
    cache: HashMap<K, PriceState>,
}

impl<K> PriceCache<K>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        PriceCache {
            cache: HashMap::new(),
        }
    }

    pub fn get<Q>(&self, id: &Q) -> Option<&PriceState>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.cache.get(id)
    }

    pub fn set_available(&mut self, id: K, value: Decimal) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Available(value))
    }

    pub fn set_unavailable(&mut self, id: K) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Unavailable)
    }

    pub fn set_unsupported(&mut self, id: K) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Unsupported)
    }

    pub fn contains<Q>(&self, id: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.cache.contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_available() {
        let mut cache = PriceCache::new();
        let id = "BTC";
        let value = Decimal::new(69420, 0);

        cache.set_available(id, value);
        assert_eq!(cache.get(&id), Some(&PriceState::Available(value)));
    }

    #[test]
    fn test_set_unavailable() {
        let mut cache = PriceCache::new();
        let id = "BTC";

        cache.set_unavailable(id);
        assert_eq!(cache.get(&id), Some(&PriceState::Unavailable));
    }

    #[test]
    fn test_set_unsupported() {
        let mut cache = PriceCache::new();
        let id = "BTC";

        cache.set_unsupported(id);
        assert_eq!(cache.get(&id), Some(&PriceState::Unsupported));
    }

    #[test]
    fn test_empty() {
        let cache: PriceCache<&str> = PriceCache::new();

        assert_eq!(cache.get(&"BTC"), None);
    }
}
