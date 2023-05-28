use crate::file_metadata::FileMetadata;
use std::collections::HashMap;

pub fn file_search(query: &str, file_index: &HashMap<String, FileMetadata>) {
    let mut results: Vec<&String> = Vec::new();
    for (path, metadata) in file_index {
        if path.contains(query) {
            results.push(path);
        }
    }
    println!("Found {} results", results.len());
    for result in results {
        println!("{}", result);
    }
}
