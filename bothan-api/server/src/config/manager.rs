//! Bothan API server manager configuration.
//!
//! Settings for manager components (e.g., crypto info manager).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::manager::ManagerConfig;
//! let config = ManagerConfig::default();
//! ```

use crypto_info::CryptoInfoManagerConfig;
use forex_info::ForexInfoManagerConfig;
use serde::{Deserialize, Serialize};

/// Shared Band worker serde helpers.
pub(crate) mod band_serde;
/// Crypto info manager configuration module.
pub mod crypto_info;
/// Forex info manager configuration module.
pub mod forex_info;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ManagerConfig {
    /// The configuration for the crypto info manager.
    pub crypto: CryptoInfoManagerConfig,
    /// The configuration for the forex info manager.
    #[serde(default)]
    pub forex: Option<ForexInfoManagerConfig>,
}
