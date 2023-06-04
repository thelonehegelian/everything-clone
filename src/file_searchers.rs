use crate::file_metadata::FileMetadata;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

// @note makes the search slower
fn wildcard_search(query: &str) -> Regex {
    let wildcard_pattern = query.replace("*", ".*").replace("?", ".");
    // let regex = Regex::new(&wildcard_pattern).unwrap();
    Regex::new(&wildcard_pattern).unwrap()
}

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
pub fn file_search_parallel<'a>(
    query: &str,
    file_index: &'a HashMap<String, FileMetadata>,
) -> Vec<&'a String> {
    // @todo the search should be some sort of a wildcard search that does not search for exact matches only
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

pub fn file_search_regex<'a>(
    query: &str,
    file_index: &'a HashMap<String, FileMetadata>,
) -> Vec<&'a String> {
    // regex wildcard case insensitive
    let regex = Regex::new(&format!("(?i){}", query)).unwrap();
    let results: Vec<&'a String> = file_index
        .par_iter()
        .filter(|(path, _)| regex.is_match(path))
        .map(|(path, _)| path)
        .collect();

    results
}
