# 🧸 ShorterDB

ShorterDB is a lightweight, embedded key-value store inspired by popular databases like RocksDB and LevelDB. It is designed to provide a simple and extensible architecture for learning and experimentation. While it may not match the performance of production-grade systems, it offers a clear and modular implementation of key-value store concepts, including Write-Ahead Logging (WAL), Memtables, and Sorted String Tables (SSTs).



## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Features](#features)
4. [Examples](#examples)
   - [Embedded Database](#embedded-database)
   - [gRPC Server](#grpc-server)
   - [CSV Import with REPL](#csv-import-with-repl)
5. [Code Walkthrough](#code-walkthrough)
   - [Error Handling](#error-handling)
   - [Database Core (`ShorterDB`)](#database-core-shorterdb)
6. [Limitations](#limitations)
7. [Future Work](#future-work)
8. [Architecture Overview](#architecture-overview)
   - [Write-Ahead Log (WAL)](#write-ahead-log-wal)
   - [Memtable](#memtable)
   - [Sorted String Table (SST)](#sorted-string-table-sst)
9. [Conclusion](#conclusion)
10. [Contributing](#contributing)

---

## Installation

To use ShorterDB in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
shorterdb = "0.1.0"
```

For building the project locally, ensure you have Rust installed. Clone the repository and run:

```bash
git clone https://github.com/your-repo/shorterdb.git
cd shorterdb
cargo build
```

---

## Introduction

ShorterDB is a simple key-value store built using a De-LSM architecture. It is designed for educational purposes and provides a modular implementation of database components. The project includes examples for embedded usage, gRPC-based remote access, and CSV imports.

---

## Features

- **Embedded Database**: Use ShorterDB as a lightweight, file-based key-value store.
- **gRPC Server**: Access the database remotely using gRPC.
- **REPL Interface**: Interact with the database in a command-line interface.
- **Write-Ahead Logging (WAL)**: Ensure durability by logging all writes.
- **Memtable**: An in-memory data structure for fast reads and writes.
- **Sorted String Table (SST)**: Persistent storage for key-value pairs.

---

## Examples

### Embedded Database

The [`embedded`](examples/embedded) example demonstrates how to use ShorterDB as an embedded database.

```rust
let mut db = ShorterDB::new(Path::new("./embedded_db")).unwrap();
db.set(b"hello", b"world").unwrap();
let value = db.get(b"hello").unwrap();
assert_eq!(value, Some(b"world".to_vec()));
```

### gRPC Server

The [`grpc`](examples/grpc) example provides a gRPC interface for remote database access.

```rust
#[tonic::async_trait]
impl Basic for DbOperations {
    async fn get(&self, request: tonic::Request<GetRequest>) -> Result<tonic::Response<GetResponse>, tonic::Status> {
        let key = request.get_ref().key.clone();
        let db = self.db.lock().await;
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => Ok(tonic::Response::new(GetResponse { value: String::from_utf8(value).unwrap() })),
            Ok(None) => Err(tonic::Status::not_found("Key not found")),
            Err(_) => Err(tonic::Status::internal("Error reading from the database")),
        }
    }
}
```

### CSV Import with REPL

The [`repl_csv`](examples/repl_csv) example imports data from a CSV file and provides a REPL interface.

```rust
let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(File::open("data.csv").unwrap());
for result in rdr.records() {
    let record = result.unwrap();
    db.set(record.get(0).unwrap().as_bytes(), record.get(1).unwrap().as_bytes()).unwrap();
}
```

---

## Code Walkthrough

### Error Handling

ShorterDB uses the `thiserror` crate for error handling. Custom error types are defined in `errors.rs`.

```rust
#[derive(Error, Debug)]
pub enum ShortDBErrors {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Value not set")]
    ValueNotSet,
    #[error("Flush needed from Memtable")]
    FlushNeededFromMemTable,
}
```

### Database Core (`ShorterDB`)

The `ShorterDB` struct ties together the Memtable and SST components.

```rust
pub struct ShorterDB {
    pub(crate) memtable: Memtable,
    pub(crate) sst: SST,
    pub(crate) data_dir: PathBuf,
}

impl ShorterDB {
    pub fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.memtable.set(key, value)?;
        Ok(())
    }
}
```

---

## Code Walkthrough

### Error Handling

ShorterDB uses the `thiserror` crate for error handling. Custom error types are defined in `errors.rs`.

```rust
#[derive(Error, Debug)]
pub enum ShortDBErrors {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Value not set")]
    ValueNotSet,
    #[error("Flush needed from Memtable")]
    FlushNeededFromMemTable,
}
```

### Database Core (`ShorterDB`)

The `ShorterDB` struct ties together the WAL, Memtable, and SST components.

```rust
pub struct ShorterDB {
    pub(crate) memtable: Memtable,
    pub(crate) wal: WAL,
    pub(crate) sst: SST,
    pub(crate) data_dir: PathBuf,
}

impl ShorterDB {
    pub fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let entry = WALEntry {
            key: Bytes::copy_from_slice(key),
            value: Bytes::copy_from_slice(value),
        };
        self.wal.write(&entry)?;
        self.memtable.set(key, value)?;
        Ok(())
    }
}
```

---

## Examples

### Embedded Database

The `embedded` example demonstrates how to use ShorterDB as an embedded database.

```rust
let mut db = ShorterDB::new(Path::new("./embedded_db")).unwrap();
db.set(b"hello", b"world").unwrap();
let value = db.get(b"hello").unwrap();
assert_eq!(value, Some(b"world".to_vec()));
```

### gRPC Server

The `grpc` example provides a gRPC interface for remote database access.

```rust
#[tonic::async_trait]
impl Basic for DbOperations {
    async fn get(&self, request: tonic::Request<GetRequest>) -> Result<tonic::Response<GetResponse>, tonic::Status> {
        let key = request.get_ref().key.clone();
        let db = self.db.lock().await;
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => Ok(tonic::Response::new(GetResponse { value: String::from_utf8(value).unwrap() })),
            Ok(None) => Err(tonic::Status::not_found("Key not found")),
            Err(_) => Err(tonic::Status::internal("Error reading from the database")),
        }
    }
}
```

### CSV Import with REPL

The `repl_csv` example imports data from a CSV file and provides a REPL interface.

```rust
let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(File::open("data.csv").unwrap());
for result in rdr.records() {
    let record = result.unwrap();
    db.set(record.get(0).unwrap().as_bytes(), record.get(1).unwrap().as_bytes()).unwrap();
}
```

---

## Limitations

- Performance is not optimized for production use.
- Limited concurrency support.
- No advanced features like compression or compaction.

---

## Future Work

- Add support for compression.
- Implement advanced compaction strategies.
- Improve concurrency and parallelism.

---

## Architecture Overview

ShorterDB is built using a modular architecture that separates concerns into distinct components:

### Write-Ahead Log (WAL)

The WAL ensures durability by logging all write operations before they are applied to the in-memory `Memtable`. This guarantees that data can be recovered in case of a crash.

```rust
pub(crate) struct WAL {
    path: PathBuf,
    file: File,
}

impl WAL {
    pub(crate) fn write(&mut self, entry: &WALEntry) -> io::Result<()> {
        self.file.write_all(&entry.key.len().to_le_bytes())?;
        self.file.write_all(entry.key.as_ref())?;
        self.file.write_all(&entry.value.len().to_le_bytes())?;
        self.file.write_all(entry.value.as_ref())?;
        self.file.flush()?;
        Ok(())
    }
}
```

### Memtable

The `Memtable` is an in-memory data structure that stores key-value pairs. It uses a `SkipMap` for efficient lookups and maintains a size limit to trigger flushing to SSTs.

```rust
pub(crate) struct Memtable {
    pub(crate) memtable: Arc<SkipMap<Bytes, Bytes>>,
    pub(crate) size: u64,
}

impl Memtable {
    pub(crate) fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.memtable.insert(Bytes::copy_from_slice(key), Bytes::copy_from_slice(value));
        self.size += 1;
        if self.size >= 256 {
            return Err(ShortDBErrors::FlushNeededFromMemTable);
        }
        Ok(())
    }
}
```

### Sorted String Table (SST)

The SST is a persistent, sorted, and immutable data structure stored on disk. It is used for long-term storage of key-value pairs.

```rust
pub(crate) struct SST {
    pub(crate) dir: PathBuf,
    pub(crate) levels: Vec<PathBuf>,
    pub(crate) queue: VecDeque<Memtable>,
}

impl SST {
    pub(crate) fn set(&mut self) {
        let mem = self.queue.pop_front().unwrap();
        for entry in mem.memtable.iter() {
            let key = entry.key();
            let value = entry.value();
            let mut path_of_kv_file = self.dir.clone();
            path_of_kv_file.push("l0");
            path_of_kv_file.push(bytes_to_string(key));
            let mut file = File::create_new(&path_of_kv_file);
            file.unwrap().write_all(value).unwrap();
        }
    }
}
```

---

## Limitations

- Performance is not optimized for production use.
- Limited concurrency support.
- No advanced features like compression or compaction.

---

## Future Work

- Add support for compression.
- Implement advanced compaction strategies.
- Improve concurrency and parallelism.

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a clear description of your changes.

---

## Conclusion

ShorterDB is a simple and modular key-value store designed for learning and experimentation. While it may not match the performance of production-grade systems, it provides a clear and extensible implementation of database concepts. Explore the [examples](#examples) to get started!
