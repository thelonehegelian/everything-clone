use crate::file_metadata::FileMetadata;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub fn display_results(results: Vec<&String>) {
    println!("Number of results: {}", results.len());
    for result in results {
        println!("{}", result);
    }
}

// @note this is a wildcard search which uses rayon to search the index
// more than twice as fast than than not using rayon
pub fn file_search_regex<'a>(
    query: &str,
    file_index: &'a HashMap<String, FileMetadata>,
) -> Vec<&'a String> {
    // @note regex wildcard case insensitive
    let regex = Regex::new(&format!("(?i){}", query)).unwrap();
    let results: Vec<&'a String> = file_index
        .par_iter()
        .filter(|(path, _)| regex.is_match(path))
        .map(|(path, _)| path)
        .collect();

    results
}

// pub fn folder_search_regex<'a>(query: &str, file_index: &'a HashMap<String, FileMetadata>) {}
