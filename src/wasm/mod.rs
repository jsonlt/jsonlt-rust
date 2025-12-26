//! WebAssembly bindings for JSONLT.
//!
//! This module provides JavaScript/WebAssembly bindings using wasm-bindgen.

use wasm_bindgen::prelude::*;

/// A JSONLT table accessible from JavaScript.
#[wasm_bindgen]
pub struct JsTable {
    inner: crate::Table,
}

#[wasm_bindgen]
impl JsTable {
    /// Creates a new empty table.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: crate::Table::new(),
        }
    }

    /// Returns the number of records in the table.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn length(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the table contains no records.
    #[wasm_bindgen(js_name = isEmpty)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns `true` if the table contains a record with the specified key.
    #[wasm_bindgen(js_name = hasKey)]
    #[must_use]
    pub fn has_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    /// Gets a record by key as a JSON string.
    ///
    /// Returns `undefined` if the key doesn't exist.
    #[wasm_bindgen]
    #[cfg(feature = "serde")]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner
            .get(key)
            .map(|record| serde_json::to_string(record.value()).unwrap_or_default())
    }

    /// Inserts a record from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid.
    #[wasm_bindgen]
    #[cfg(feature = "serde")]
    pub fn insert(&mut self, key: &str, json: &str) -> Result<(), JsValue> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner
            .insert(key.to_string(), crate::Record::from_value(value));
        Ok(())
    }

    /// Removes a record by key.
    ///
    /// Returns `true` if the record was removed, `false` if it didn't exist.
    #[wasm_bindgen]
    pub fn remove(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }

    /// Returns all keys as a JSON array string.
    #[wasm_bindgen]
    #[cfg(feature = "serde")]
    #[must_use]
    pub fn keys(&self) -> String {
        let keys: Vec<&String> = self.inner.keys().collect();
        serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string())
    }
}

impl Default for JsTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Initializes the WASM module.
///
/// This function should be called once when the module is loaded.
#[wasm_bindgen(start)]
#[allow(clippy::missing_const_for_fn)] // wasm_bindgen(start) requires non-const
pub fn init() {
    // Placeholder for initialization logic
    // Future: Add panic hook for better error messages in the browser console
}
