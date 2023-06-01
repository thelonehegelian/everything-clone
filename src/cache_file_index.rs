use crate::file_indexers::create_hash_index_parallel;
use crate::file_metadata::FileMetadata;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn create_and_cache_file_index(
) -> Result<HashMap<String, FileMetadata>, Box<dyn std::error::Error>> {
    match fs::metadata("file_index_cache.json") {
        Ok(metadata) if metadata.is_file() => {
            println!("Loading cached file index from file_index_cache.json");
            return load_cached_file_index();
        }
        Ok(_) => {
            println!("file_index_cache.json found, but it is not a regular file");
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("file_index_cache.json not found");
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    println!("Creating file index and caching it in file_index_cache.json");
    let file_index = create_hash_index_parallel();

    // Convert the file index to JSON
    let json = serde_json::to_string(&file_index)?;

    // Write the JSON to a file
    let mut file = File::create("file_index_cache.json")?;
    file.write_all(json.as_bytes())?;

    Ok(file_index)
}

fn load_cached_file_index() -> Result<HashMap<String, FileMetadata>, Box<dyn std::error::Error>> {
    // Check if the file exists
    // @todo uncomment this after creating a command line argument to create the file index
    // if !fs::metadata("file_index.json")?.is_file() {
    //     return Err("File index not found".into());
    // }

    // Read the JSON from the file
    let file = File::open("file_index_cache.json")?;
    let reader = io::BufReader::new(file);
    let json: String = reader.lines().collect::<Result<_, _>>()?;

    // Parse the JSON into the file index hashmap
    let file_index: HashMap<String, FileMetadata> = serde_json::from_str(&json)?;

    Ok(file_index)
}
