//! # Helper Traits for CLI Error Handling
//!
//! This module provides utility traits for error handling in the Bothan API CLI.
//! In particular, it defines the `Exitable` trait, which allows for convenient
//! process exit on error results.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use crate::helper::Exitable;
//!
//! let result: anyhow::Result<()> = do_something();
//! result.exit_on_err(1);
//! ```

/// A trait for exiting the process on error results.
pub trait Exitable<T> {
    /// Exits the process with the given code if the result is an error.
    ///
    /// # Arguments
    ///
    /// * `code` - The exit code to use if an error occurs.
    ///
    /// # Returns
    ///
    /// Returns the value if `Ok`, otherwise prints the error and exits.
    fn exit_on_err(self, code: i32) -> T;
}

impl<T> Exitable<T> for anyhow::Result<T> {
    fn exit_on_err(self, code: i32) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(code);
            }
        }
    }
}
