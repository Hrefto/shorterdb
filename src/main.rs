pub mod errors;
pub mod kv;
use anyhow::Result;
use clap::{Parser, Subcommand};
use csv::ReaderBuilder;
use kv::db::ShorterDB;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "shortdb")]
#[command(about = "A simple key-value store REPL", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

fn main() -> Result<()> {
    let mut db = ShorterDB::new(Path::new("./test_db"))?;

    // Read data from CSV file
    let csv_file_path = PathBuf::from("data.csv");
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(csv_file_path)?);

    for result in rdr.records() {
        let record = result?;
        if record.len() == 2 {
            let key = record.get(0).unwrap();
            let value = record.get(1).unwrap();
            db.set(key.as_bytes(), value.as_bytes())?;
            println!("Inserted Key: {}, Value: {}", key, value);
        }
    }

    println!("Welcome to the ShortDB REPL!");
    println!("Syntax:- \n (i) set <key> <value> : maps <key> and <value> \n ");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        // Parse the input command using clap
        let args: Vec<&str> = input.split_whitespace().collect();
        let cli = Cli::parse_from(args);

        match cli.command {
            Some(Commands::Set { key, value }) => {
                db.set(key.as_bytes(), value.as_bytes())?;
                println!("Key: {}, Value: {} Set", key, value);
            }
            Some(Commands::Get { key }) => {
                match db.get(key.as_bytes()) {
                    Ok(Some(v)) => {
                        println!("Value for key: {} found: {:?}", &key, String::from_utf8(v));
                    }
                    Ok(None) => {
                        println!("The value for key:{}, was deleted", key);
                    }
                    Err(errors::ShortDBErrors::KeyNotFound) => {
                        println!("Value for Key: {} Not found!!", &key);
                    }
                    Err(e) => println!("Some error happened, {}", e),
                };
            }
            Some(Commands::Delete { key }) => match db.delete(&key.as_bytes()) {
                Ok(()) => {
                    println!("Value for key: {} changed to tombstone", key);
                }
                Err(e) => {
                    println!("Some error happened {}", e);
                }
            },
            None => println!("Unknown command. Use 'get', 'set' or 'delete'."),
        }
    }

    println!("Exiting the REPL. Goodbye!");
    Ok(())
}
