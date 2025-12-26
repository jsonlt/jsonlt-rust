//! Shared test utilities for JSONLT tests.

use jsonlt::Table;

/// Creates a new empty table for testing.
#[must_use]
pub fn create_test_table() -> Table {
    Table::new()
}

/// Creates a table with sample records for testing.
#[cfg(feature = "serde")]
#[must_use]
pub fn create_sample_table() -> Table {
    use jsonlt::Record;
    use serde_json::json;

    let mut table = Table::new();
    table.insert(
        "user-1".to_string(),
        Record::from_value(json!({
            "name": "Alice",
            "email": "alice@example.com"
        })),
    );
    table.insert(
        "user-2".to_string(),
        Record::from_value(json!({
            "name": "Bob",
            "email": "bob@example.com"
        })),
    );
    table
}

/// Asserts that two tables have the same keys.
pub fn assert_same_keys(left: &Table, right: &Table) {
    let mut left_keys: Vec<_> = left.keys().collect();
    let mut right_keys: Vec<_> = right.keys().collect();
    left_keys.sort();
    right_keys.sort();
    assert_eq!(left_keys, right_keys);
}
