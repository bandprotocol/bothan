use std::collections::HashMap;

use crate::registry::signal::Signal;

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;

pub type Registry = HashMap<String, Signal>;
