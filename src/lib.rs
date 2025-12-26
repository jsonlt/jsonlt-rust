//! # JSON Lines Table (JSONLT)
//!
//! A library for using a JSON Lines (JSONL) file as a lightweight database.
//!
//! JSONLT (JSON Lines Table) is a data format specification for storing keyed records
//! in append-only files using [JSON Lines](https://jsonlines.org/). Optimized for
//! version control diffs and human readability.
//!
//! ## Features
//!
//! - **serde** (default): Enables serialization/deserialization support via serde
//! - **async**: Enables async I/O operations via tokio
//! - **derive**: Enables derive macros for error types
//! - **wasm**: Enables WebAssembly bindings
//! - **full**: Enables all features
//!
//! ## Example
//!
//! ```rust
//! use jsonlt::Table;
//!
//! // Create a new table
//! let table = Table::new();
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod error;
mod ops;
mod record;
mod table;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod io;

#[cfg(feature = "wasm")]
#[cfg_attr(docsrs, doc(cfg(feature = "wasm")))]
pub mod wasm;

pub use error::{Error, Result};
pub use ops::Operations;
pub use record::Record;
pub use table::Table;
