use std::error::Error as StdError;
use std::fmt;

/// A custom error type that wraps another error with an optional message.
pub struct AssetWorkerError {
    pub msg: String,
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}

impl AssetWorkerError {
    /// Create a new `AssetWorkerError` with a message.
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            source: None,
        }
    }

    /// Create a new `AssetWorkerError` with a message and a source error.
    pub fn with_source<E>(msg: impl Into<String>, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
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
    E: StdError + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        Self {
            msg: format!("An error occurred: {}", err),
            source: Some(Box::new(err)),
        }
    }
}
