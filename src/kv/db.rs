use super::{
    memtable::Memtable,
    sst::SST,
    wal::{WALEntry, WAL},
};
use crate::errors::{Result, ShortDBErrors};
use bytes::Bytes;
use std::path::{Path, PathBuf};
use std::{fs, io::Read};

pub struct ShorterDB {
    pub(crate) memtable: Memtable,
    pub(crate) wal: WAL,
    pub(crate) sst: SST,
    pub(crate) data_dir: PathBuf,
}

impl ShorterDB {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        fs::create_dir_all(&data_dir); // Ensure the data directory exists

        let wal = WAL::new(&data_dir).unwrap();
        let sst = SST::open("db_test".to_string());

        Ok(Self {
            memtable: Memtable::new(),
            wal,
            sst,
            data_dir,
        })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // First check in Memtable
        // if let Some(value) = self.memtable.get(key) {
        //     return Ok(Some(value));
        // }
        //

        match self.memtable.get(key) {
            Ok(None) => println!("data deleted"),
            Ok(Some(v)) => {
                return Ok(Some(v.to_vec()));
            }
            Err(ShortDBErrors::KeyNotFound) => println!("not found in mem"),
            Err(e) => println!("something problematic happend {}", e),
        }

        // If not found in Memtable, check SST
        if let Some(value) = self.sst.get(key) {
            print!("checking in sst");
            return Ok(Some(value));
        }

        Err(ShortDBErrors::KeyNotFound) // Return None if not found
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        // Create a new WALEntry
        let entry = WALEntry {
            key: Bytes::copy_from_slice(key),
            value: Bytes::copy_from_slice(value),
        };

        // Write to the WAL
        self.wal.write(&entry);

        // Insert into Memtable
        self.memtable.set(key, value)?;

        // Check if we need to flush Memtable to SST
        if let Err(err) = self.memtable.set(key, value) {
            match err {
                ShortDBErrors::FlushNeededFromMemTable => self.flush_memtable()?,
                _ => println!("some err happend"),
            }
        }

        Ok(())
    }

    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        // Create a tombstone entry
        let tombstone_entry = WALEntry {
            key: Bytes::copy_from_slice(key),
            value: Bytes::copy_from_slice(b"tombstone"),
        };

        // Write tombstone to WAL
        self.wal.write(&tombstone_entry);

        // Delete from Memtable
        self.memtable.delete(key)?;

        // Check if we need to flush Memtable to SST
        if let Err(err) = self.memtable.delete(key) {
            match err {
                ShortDBErrors::FlushNeededFromMemTable => self.flush_memtable()?,
                _er => println!("some problem: {}", _er),
            }
        }

        Ok(())
    }

    fn flush_memtable(&mut self) -> Result<()> {
        // Flush entries from Memtable to SST
        // for entry in self.memtable.memtable.iter() {
        //     self.sst.set(entry.key().as_ref(), entry.value().as_ref())?;
        // }
        self.sst.queue.push_back(self.memtable.clone());
        self.sst.set();

        // Clear the Memtable after flushing
        self.memtable.clear();

        Ok(())
    }
}
