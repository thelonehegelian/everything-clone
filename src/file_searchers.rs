use crate::file_metadata::FileMetadata;
use rayon::prelude::*;
use std::collections::HashMap;

// @todo this should be a parallel search function that uses rayon to search the index
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

// @note more than twice as fast than the file_search function
pub fn file_seach_parallel<'a>(
    query: &str,
    file_index: &'a HashMap<String, FileMetadata>,
) -> Vec<&'a String> {
    let results: Vec<&'a String> = file_index
        .par_iter()
        .filter(|(path, _)| path.contains(query))
        .map(|(path, _)| path)
        .collect();

    results
}

pub fn display_results(results: Vec<&String>) {
    println!("Number of results: {}", results.len());
    for result in results {
        println!("{}", result);
    }
}
