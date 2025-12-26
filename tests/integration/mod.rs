//! Integration tests for JSONLT.

mod common;

use jsonlt::{Operations, Record, Table};

#[test]
fn test_table_basic_operations() {
    let mut table = Table::new();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);

    // Insert a record
    table.insert("key-1".to_string(), Record::new());
    assert!(!table.is_empty());
    assert_eq!(table.len(), 1);
    assert!(table.contains_key("key-1"));
    assert!(!table.contains_key("key-2"));

    // Remove a record
    let removed = table.remove("key-1");
    assert!(removed.is_some());
    assert!(table.is_empty());
}

#[test]
fn test_operations_insert() {
    let mut table = Table::new();

    let op = Operations::Insert {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_ok());
    assert!(table.contains_key("test-key"));
}

#[test]
fn test_operations_insert_duplicate() {
    let mut table = Table::new();
    table.insert("test-key".to_string(), Record::new());

    let op = Operations::Insert {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_err());
}

#[test]
fn test_operations_update() {
    let mut table = Table::new();
    table.insert("test-key".to_string(), Record::new());

    let op = Operations::Update {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_ok());
}

#[test]
fn test_operations_update_nonexistent() {
    let mut table = Table::new();

    let op = Operations::Update {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_err());
}

#[test]
fn test_operations_delete() {
    let mut table = Table::new();
    table.insert("test-key".to_string(), Record::new());

    let op = Operations::Delete {
        key: "test-key".to_string(),
    };

    assert!(op.apply(&mut table).is_ok());
    assert!(!table.contains_key("test-key"));
}

#[test]
fn test_operations_delete_nonexistent() {
    let mut table = Table::new();

    let op = Operations::Delete {
        key: "test-key".to_string(),
    };

    assert!(op.apply(&mut table).is_err());
}

#[test]
fn test_operations_upsert_insert() {
    let mut table = Table::new();

    let op = Operations::Upsert {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_ok());
    assert!(table.contains_key("test-key"));
}

#[test]
fn test_operations_upsert_update() {
    let mut table = Table::new();
    table.insert("test-key".to_string(), Record::new());

    let op = Operations::Upsert {
        key: "test-key".to_string(),
        record: Record::new(),
    };

    assert!(op.apply(&mut table).is_ok());
    assert!(table.contains_key("test-key"));
}

#[cfg(feature = "serde")]
mod serde_tests {
    use jsonlt::{Record, Table};
    use serde_json::json;

    #[test]
    fn test_record_from_json_value() {
        let value = json!({
            "name": "Test",
            "count": 42
        });

        let record = Record::from_value(value.clone());
        assert_eq!(record.value(), &value);
    }

    #[test]
    fn test_table_with_json_records() {
        let mut table = Table::new();

        table.insert(
            "item-1".to_string(),
            Record::from_value(json!({"name": "First"})),
        );
        table.insert(
            "item-2".to_string(),
            Record::from_value(json!({"name": "Second"})),
        );

        assert_eq!(table.len(), 2);
        assert_eq!(
            table.get("item-1").unwrap().value(),
            &json!({"name": "First"})
        );
    }
}
