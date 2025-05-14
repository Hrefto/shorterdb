//! # ShorterDB
//!
//! ShorterDB is a simple key-value store built using SkipLists and a De-LSM architecture.
//! It can be used as an embedded database or as a gRPC-enabled server.
//!
//! ## Features
//! - Embedded database with `ShorterDB`.
//! - gRPC server for remote database access.
//! - REPL for interactive usage.
//!
//! ## Usage
//!
//! Add `ShorterDB` to your `Cargo.toml`:
//! ```toml
//! shorterdb = "0.1.0"
//! ```
//!
//! ### Embedded Database
//! ```rust
//! use shorterdb::kv::db::ShorterDB;
//! use std::path::Path;
//!
//! let mut db = ShorterDB::new(Path::new("./test_db")).unwrap();
//! db.set(b"key1", b"value1").unwrap();
//! let value = db.get(b"key1").unwrap();
//! assert_eq!(value, Some(b"value1".to_vec()));
//! ```

pub mod errors;
pub mod kv;

pub use kv::db::ShorterDB;
