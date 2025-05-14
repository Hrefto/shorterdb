use shorterdb::ShorterDB;
use std::path::Path;

fn main() {
    // Initialize the embedded ShorterDB
    let mut db = ShorterDB::new(Path::new("./embedded_db")).expect("Failed to initialize database");

    // Store "hello" and "world" in the database
    db.set(b"hello", b"world")
        .expect("Failed to set key-value pair");

    // Retrieve the value for "hello"
    match db.get(b"hello") {
        Ok(Some(value)) => {
            println!(
                "Key: 'hello', Value: '{}'",
                String::from_utf8(value).unwrap()
            );
        }
        Ok(None) => {
            println!("The value for key 'hello' was deleted or not found");
        }
        Err(_) => {
            println!("Error retrieving value for key 'hello'");
        }
    }

    // Clean up
    println!("Embedded ShorterDB example completed.");
}
