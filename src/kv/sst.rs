use std::{
    collections::VecDeque,
    fs::{self, create_dir, remove_file, OpenOptions},
    io::Write,
    path::{self, Path, PathBuf},
};

use std::fs::File;
use std::io::{BufReader, Read};

// use anyhow::Error;

use anyhow::Error;

use super::{memtable::Memtable, utils::bytes_to_string};

pub struct SST {
    pub dir: PathBuf,
    pub levels: Vec<PathBuf>,
    pub max_level_size: Vec<usize>,
    pub curr_level_size: Vec<usize>,
    pub queue: VecDeque<Memtable>,
    // parralellisation: todo!(),
}

impl SST {
    pub fn open(db_name: String) -> Self {
        let dir;
        match create_dir("./".to_string() + &db_name) {
            Ok(()) => {
                //dir had to be created.
                dir = PathBuf::from("./".to_string() + &db_name);
                let mut l0 = dir.clone();
                l0.push("./l0");
                create_dir(l0.clone());
                let mut levels = Vec::new();
                levels.push(l0.clone());
                let mut max_level_size = Vec::new();
                max_level_size.push(1024);
                let mut curr_level_size = Vec::new();
                curr_level_size.push(0);
                SST {
                    dir,
                    levels,
                    max_level_size,
                    queue: VecDeque::new(),
                    curr_level_size,
                }
            }
            Err(e) => {
                //dir already exists
                dir = PathBuf::from("./".to_string() + &db_name);

                let children = dir.read_dir().unwrap();
                let mut levels = Vec::new();
                let mut curr_level_size = Vec::new();
                let mut max_level_size = Vec::new();
                let mut i: usize = 0;
                for child in children {
                    let child = child.unwrap();
                    let path = child.path();
                    if path.is_dir() {
                        let level = path.clone();
                        levels.push(path.clone());
                        max_level_size.push(1024 * 10_i32.pow(i as u32) as usize);
                        let curr_no_of_kvs_in_level = path.read_dir().unwrap().count();
                        max_level_size[i] = curr_no_of_kvs_in_level;
                        i += 1;
                    }
                }

                SST {
                    dir,
                    levels,
                    max_level_size,
                    curr_level_size,
                    queue: VecDeque::new(),
                }
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        dbg!("looking in sst");
        //get from sst
        for level in self.levels.iter() {
            dbg!(&level);
            //get from level
            let ssts: fs::ReadDir = level.read_dir().unwrap();
            for sst in ssts {
                let mut directory = level.clone();
                let name = bytes_to_string(key);
                directory.push(name.clone());
                //get from sst
                match Path::new(&directory).try_exists() {
                    Ok(true) => {
                        //file exists in this sst folder
                        match fs::read(directory) {
                            Ok(val) => {
                                return Some(val);
                            }
                            Err(e) => {
                                println!("some error happend{}", e);
                            }
                        }
                    }
                    Ok(false) => {
                        //continue
                    }
                    Err(e) => {
                        println!("error while seeking into sst files{}", e);
                    }
                }
            }
        }
        return None;
    }

    pub fn set(&mut self) {
        let mem = self.queue.pop_front().unwrap();

        // Use iter() and access key and value from each entry
        for entry in mem.memtable.iter() {
            let key = entry.key();
            let value = entry.value();

            let mut path_of_kv_file = self.dir.clone();
            path_of_kv_file.push("l0");
            self.curr_level_size.push(0);
            match path_of_kv_file.is_dir() {
                false => {
                    create_dir(&path_of_kv_file).expect("sorry couldnt create the folder");
                }
                true => {
                    print!("folder already there");
                }
            }
            path_of_kv_file.push(bytes_to_string(key));
            dbg!(&path_of_kv_file);
            let mut file = File::create_new(&path_of_kv_file);
            match file {
                Ok(_) => {
                    //file was not there
                    file.unwrap().write_all(value).unwrap();
                }
                Err(_) => {
                    //file was there overwrite file
                    print!("most probably already existing");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(path_of_kv_file);
                    file.unwrap().write_all(value);
                }
            };

            self.curr_level_size[0] += 1;
            if self.curr_level_size >= self.max_level_size {
                self.compact();
            }
        }
    }

    pub fn compact(&self) {
        print!("ok compacted");
        // todo!()
    }

    pub fn delete(&mut self, key: &[u8]) -> Result<(), Error> {
        for level in self.levels.iter() {
            dbg!(&level);
            let ssts: fs::ReadDir = level.read_dir()?;
            for sst in ssts {
                let mut directory = level.clone();
                let name = bytes_to_string(key);
                directory.push(name.clone());
                if Path::new(&directory).exists() {
                    remove_file(&directory)?;
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
