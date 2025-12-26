//! Table type representing a JSONLT table.

use crate::Record;
use std::collections::HashMap;

/// A JSONLT table containing keyed records.
///
/// The table stores records in an append-only manner, optimized for
/// version control diffs and human readability.
#[derive(Debug, Clone, Default)]
pub struct Table {
    /// The records in the table, keyed by their identifier.
    records: HashMap<String, Record>,
}

impl Table {
    /// Creates a new empty table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsonlt::Table;
    ///
    /// let table = Table::new();
    /// assert!(table.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of records in the table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` if the table contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns a reference to a record by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Record> {
        self.records.get(key)
    }

    /// Returns `true` if the table contains a record with the specified key.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.records.contains_key(key)
    }

    /// Inserts a record into the table.
    ///
    /// If a record with the same key already exists, it is replaced.
    pub fn insert(&mut self, key: String, record: Record) {
        self.records.insert(key, record);
    }

    /// Removes a record from the table by key.
    ///
    /// Returns the removed record if it existed.
    pub fn remove(&mut self, key: &str) -> Option<Record> {
        self.records.remove(key)
    }

    /// Returns an iterator over the records in the table.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Record)> {
        self.records.iter()
    }

    /// Returns an iterator over the keys in the table.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.records.keys()
    }
}
