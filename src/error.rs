//! Error types for JSONLT operations.

use std::fmt;

/// A specialized Result type for JSONLT operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for JSONLT operations.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),

    /// A JSON parsing error occurred.
    #[cfg(feature = "serde")]
    Json(serde_json::Error),

    /// A record with the specified key was not found.
    NotFound(String),

    /// A record with the specified key already exists.
    AlreadyExists(String),

    /// The record format is invalid.
    InvalidFormat(String),

    /// The key format is invalid.
    InvalidKey(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            #[cfg(feature = "serde")]
            Self::Json(err) => write!(f, "JSON error: {err}"),
            Self::NotFound(key) => write!(f, "record not found: {key}"),
            Self::AlreadyExists(key) => write!(f, "record already exists: {key}"),
            Self::InvalidFormat(msg) => write!(f, "invalid format: {msg}"),
            Self::InvalidKey(msg) => write!(f, "invalid key: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            #[cfg(feature = "serde")]
            Self::Json(err) => Some(err),
            Self::NotFound(_)
            | Self::AlreadyExists(_)
            | Self::InvalidFormat(_)
            | Self::InvalidKey(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
