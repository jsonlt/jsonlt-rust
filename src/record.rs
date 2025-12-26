//! Record type representing a single JSONLT record.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A single record in a JSONLT table.
///
/// Records are stored as JSON objects with a key field that identifies them.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Record {
    /// The record data as a JSON value.
    #[cfg(feature = "serde")]
    data: serde_json::Value,

    /// The record data as raw bytes (when serde is not enabled).
    #[cfg(not(feature = "serde"))]
    data: Vec<u8>,
}

impl Record {
    /// Creates a new empty record.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a record from a JSON value.
    #[cfg(feature = "serde")]
    #[must_use]
    pub const fn from_value(value: serde_json::Value) -> Self {
        Self { data: value }
    }

    /// Returns the record data as a JSON value reference.
    #[cfg(feature = "serde")]
    #[must_use]
    pub const fn value(&self) -> &serde_json::Value {
        &self.data
    }

    /// Returns the record data as a mutable JSON value reference.
    #[cfg(feature = "serde")]
    pub fn value_mut(&mut self) -> &mut serde_json::Value {
        &mut self.data
    }

    /// Creates a record from raw bytes.
    #[cfg(not(feature = "serde"))]
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { data: bytes }
    }

    /// Returns the record data as raw bytes.
    #[cfg(not(feature = "serde"))]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Value> for Record {
    fn from(value: serde_json::Value) -> Self {
        Self::from_value(value)
    }
}
