//! Operations for JSONLT tables.

use crate::{Error, Record, Result, Table};

/// Operations that can be performed on a JSONLT table.
#[derive(Debug, Clone)]
pub enum Operations {
    /// Insert a new record.
    Insert {
        /// The key for the new record.
        key: String,
        /// The record to insert.
        record: Record,
    },

    /// Update an existing record.
    Update {
        /// The key of the record to update.
        key: String,
        /// The updated record data.
        record: Record,
    },

    /// Delete a record.
    Delete {
        /// The key of the record to delete.
        key: String,
    },

    /// Upsert a record (insert or update).
    Upsert {
        /// The key for the record.
        key: String,
        /// The record data.
        record: Record,
    },
}

impl Operations {
    /// Applies this operation to a table.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Insert is called for a key that already exists
    /// - Update or Delete is called for a key that doesn't exist
    pub fn apply(self, table: &mut Table) -> Result<()> {
        match self {
            Self::Insert { key, record } => {
                if table.contains_key(&key) {
                    return Err(Error::AlreadyExists(key));
                }
                table.insert(key, record);
            }
            Self::Update { key, record } => {
                if !table.contains_key(&key) {
                    return Err(Error::NotFound(key));
                }
                table.insert(key, record);
            }
            Self::Delete { key } => {
                if table.remove(&key).is_none() {
                    return Err(Error::NotFound(key));
                }
            }
            Self::Upsert { key, record } => {
                table.insert(key, record);
            }
        }
        Ok(())
    }
}
