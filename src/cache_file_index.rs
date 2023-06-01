use crate::file_indexers::create_hash_index_parallel;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

pub fn create_and_cache_file_index() -> Result<(), Box<dyn std::error::Error>> {
    let file_index = create_hash_index_parallel();

    // Convert the file index to JSON
    // @todo use a serialized file format instead of JSON
    let json = serde_json::to_string(&file_index)?;

    // Write the JSON to a file
    let mut file = File::create("file_index.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
