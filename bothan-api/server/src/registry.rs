use std::collections::HashMap;

use crate::registry::signal::Signal;
use crate::tasks::Tasks;
pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;

/// The `Registry` type is a HashMap that maps a signal name to its corresponding `Signal`.
pub type Registry = HashMap<String, Signal>;

pub trait Validator {
    fn validate(&self) -> bool;
}

impl Validator for Registry {
    fn validate(&self) -> bool {
        Tasks::try_from(self.clone()).is_ok()
    }
}
