// ! Error types for asset workers.
//!
//! This module provides error handling functionality for asset workers. It defines
//! a custom error type that can wrap other errors and provide additional context.
//!
//! The module provides:
//!
//! - The [`AssetWorkerError`] struct which serves as a universal error type for all worker implementations
//! - Methods for creating errors with or without source errors
//! - Implementations of standard error traits
//!
//! # Error Handling Strategy
//!
//! The error handling strategy in this module follows these principles:
//!
//! 1. **Context Preservation**: Errors include both a human-readable message and the original error
//! 2. **Error Propagation**: The `From` trait implementation allows easy conversion from other error types
//! 3. **Diagnostic Information**: Errors provide clear and helpful diagnostic information
//!
//! When implementing worker types, use this error type to provide consistent error
//! handling throughout the asset worker system.

use std::error::Error;
use std::fmt;

/// A custom error type for asset workers that wraps another error with an optional message.
///
/// `AssetWorkerError` provides a consistent error type for all asset worker implementations,
/// allowing errors to be propagated with context. It can be used either as a standalone error
/// with just a message, or it can wrap another error while adding additional context.
///
/// # Examples
///
/// Creating a standalone error:
///
/// ```
/// use bothan_lib::worker::error::AssetWorkerError;
///
/// let error = AssetWorkerError::new("Failed to initialize worker");
/// ```
///
/// Creating an error that wraps another error:
///
/// ```
/// use bothan_lib::worker::error::AssetWorkerError;
/// use std::io;
///
/// let io_error = io::Error::new(io::ErrorKind::NotFound, "Resource not found");
/// let error = AssetWorkerError::with_source("Failed to fetch asset data", io_error);
/// ```
///
/// Using the `From` trait for automatic conversion with the `?` operator for any error type that implements the `Error` trait:
///
/// ```
/// use bothan_lib::worker::error::AssetWorkerError;
/// use std::io;
///
/// fn process() -> Result<(), AssetWorkerError> {
///     // The ? operator automatically converts io::Error to AssetWorkerError using From
///     let result = std::fs::read_to_string("config.json")?;
///     
///     // Process the result...
///     Ok(())
/// }
/// ```
///
pub struct AssetWorkerError {
    /// A human-readable error message.
    pub msg: String,

    /// The optional source error that caused this error.
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl AssetWorkerError {
    /// Create a new `AssetWorkerError` with a message.
    ///
    /// This method creates a standalone error without a source error.
    /// Use this when the error originates within the worker code.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::worker::error::AssetWorkerError;
    ///
    /// let error = AssetWorkerError::new("Failed to initialize worker");
    /// ```
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            source: None,
        }
    }

    /// Create a new `AssetWorkerError` with a message and a source error.
    ///
    /// This method creates an error that wraps another error, adding context
    /// about the operation that was being performed when the error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::worker::error::AssetWorkerError;
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "Resource not found");
    /// let error = AssetWorkerError::with_source("Failed to fetch asset data", io_error);
    /// ```
    pub fn with_source<E>(msg: impl Into<String>, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            msg: msg.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for AssetWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl fmt::Debug for AssetWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<E> From<E> for AssetWorkerError
where
    E: Error + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        Self {
            msg: format!("An error occurred: {}", err),
            source: Some(Box::new(err)),
        }
    }
}
