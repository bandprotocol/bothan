//! Price cache utility for storing and retrieving price states.
//!
//! Provides the `PriceCache` struct for managing cached `PriceState` values by key.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use rust_decimal::Decimal;

use crate::manager::crypto_asset_info::types::PriceState;

/// In-memory cache for storing `PriceState` values keyed by asset or signal ID.
pub struct PriceCache<K> {
    cache: HashMap<K, PriceState>,
}

impl<K> PriceCache<K>
where
    K: Hash + Eq,
{
    /// Creates an empty `PriceCache`.
    pub fn new() -> Self {
        PriceCache {
            cache: HashMap::new(),
        }
    }

    /// Returns a reference to the `PriceState` corresponding to the given `id`.
    pub fn get<Q>(&self, id: &Q) -> Option<&PriceState>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.cache.get(id)
    }

    /// Inserts a new `PriceState::Available` with the given `id` and `value`.
    pub fn set_available(&mut self, id: K, value: Decimal) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Available(value))
    }

    /// Inserts a new `PriceState::Unavailable` with the given `id`.
    pub fn set_unavailable(&mut self, id: K) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Unavailable)
    }

    /// Inserts a new `PriceState::Unsupported` with the given `id`.
    pub fn set_unsupported(&mut self, id: K) -> Option<PriceState> {
        self.cache.insert(id, PriceState::Unsupported)
    }

    /// Returns `true` if the cache contains the given `id`.
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
