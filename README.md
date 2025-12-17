<p align="center">
  <img src="https://raw.githubusercontent.com/ApiliumCode/aingle/main/assets/aingle.svg" alt="AIngle Logo" width="200"/>
</p>

<h1 align="center">rkv</h1>

<p align="center">
  <strong>Typed key-value storage with multiple backends for AIngle</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/rkv"><img src="https://img.shields.io/crates/v/rkv.svg" alt="Crates.io"/></a>
  <a href="https://docs.rs/rkv"><img src="https://docs.rs/rkv/badge.svg" alt="Documentation"/></a>
  <a href="https://github.com/ApiliumCode/rkv/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License"/></a>
  <a href="https://github.com/ApiliumCode/rkv/actions"><img src="https://github.com/ApiliumCode/rkv/workflows/CI/badge.svg" alt="CI Status"/></a>
</p>

---

## Overview

A simple, humane, typed key-value storage solution for Rust. This crate supports multiple backend engines with varying guarantees, optimized for use within the AIngle distributed systems framework.

## Features

- **Multiple backends** - LMDB for performance, SafeMode for reliability
- **Type-safe API** - Strongly typed keys and values
- **Concurrent access** - Multi-reader, single-writer transactions
- **Crash-safe** - ACID-compliant storage with durability guarantees
- **Compact** - Efficient memory-mapped I/O

## Installation

```toml
[dependencies]
rkv = "0.1"
```

## Quick Start

```rust
use rkv::{Manager, Rkv, StoreOptions, Value};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open or create the environment
    let path = Path::new("./my-store");
    let mut manager = Manager::singleton().write()?;
    let env = manager.get_or_create(path, Rkv::new)?;

    // Open a store
    let store = env.read()?.open_single("mystore", StoreOptions::create())?;

    // Write data
    {
        let mut writer = env.read()?.write()?;
        store.put(&mut writer, "key", &Value::Str("value"))?;
        writer.commit()?;
    }

    // Read data
    let reader = env.read()?.read()?;
    let value = store.get(&reader, "key")?;
    println!("Value: {:?}", value);

    Ok(())
}
```

## Backend Engines

| Backend | Use Case | Characteristics |
|---------|----------|-----------------|
| **LMDB** | High performance | Memory-mapped, fast reads |
| **SafeMode** | Reliability | In-memory with sync writes |
| **SQLite** | Portability | Planned for future release |

### SafeMode Backend

For production environments requiring maximum reliability:

```rust
use rkv::{Manager, Rkv};
use rkv::backend::{SafeMode, SafeModeEnvironment};

let mut manager = Manager::<SafeModeEnvironment>::singleton().write()?;
let env = manager.get_or_create(path, Rkv::new::<SafeMode>)?;
```

## Features Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `db-dup-sort` | Multiple values per key | Yes |
| `db-int-key` | Integer key optimizations | Yes |
| `backtrace` | Error backtraces | No |

## Build & Test

```bash
# Build
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Run examples
./run-all-examples.sh
```

## Part of AIngle

This crate is part of the [AIngle](https://github.com/ApiliumCode/aingle) ecosystem - a Semantic DAG framework for IoT and distributed AI applications.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Maintained by <a href="https://apilium.com">Apilium Technologies</a> - Tallinn, Estonia</sub>
</p>
