<!-- vale Google.Headings = NO -->

# JSON Lines Table (JSONLT) Rust crate

<!-- vale Google.Headings = YES -->
<!-- vale off -->
[![CI](https://github.com/jsonlt/jsonlt-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/jsonlt/jsonlt-rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/jsonlt.svg)](https://crates.io/crates/jsonlt)
[![Codecov](https://codecov.io/gh/jsonlt/jsonlt-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/jsonlt/jsonlt-rust)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](https://opensource.org/licenses/MIT)
<!-- vale on -->

**jsonlt** is the Rust implementation of the [JSON Lines Table (JSONLT) specification][jsonlt].

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
jsonlt = "0.0.0"
```

Or with all features:

```toml
[dependencies]
jsonlt = { version = "0.0.0", features = ["full"] }
```

## Usage

```rust
use jsonlt::Table;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open or create a table
    let mut table = Table::open("data.jsonlt")?;

    // Add records
    table.set("user:1", json!({"name": "Alice", "age": 30}))?;
    table.set("user:2", json!({"name": "Bob", "age": 25}))?;

    // Get a record
    if let Some(user) = table.get("user:1")? {
        println!("{}", user["name"]); // "Alice"
    }

    // Check if a record exists
    println!("{}", table.has("user:1")?); // true

    // Delete a record
    table.delete("user:1")?;

    // Iterate over all records
    for (key, record) in table.iter()? {
        println!("{}: {}", key, record);
    }

    Ok(())
}
```

## Features

- **serde** (default): Enables serialization/deserialization support via serde
- **async**: Enables async I/O operations via tokio
- **derive**: Enables derive macros for error types
- **wasm**: Enables WebAssembly bindings
- **full**: Enables all features

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[jsonlt]: https://jsonlt.org/
