use crate::file_metadata::FileMetadata;
use std::collections::HashMap;

pub fn file_search<'a>(
    query: &str,
    file_index: &'a HashMap<String, FileMetadata>,
) -> Vec<&'a String> {
    let mut results: Vec<&'a String> = Vec::new();
    for (path, metadata) in file_index {
        if path.contains(query) {
            results.push(path);
        }
    }
    results
}

pub fn display_results(results: Vec<&String>) {
    println!("Number of results: {}", results.len());
    for result in results {
        println!("{}", result);
    }
}
