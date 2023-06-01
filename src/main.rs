mod file_indexers;
mod file_metadata;
use file_metadata::FileMetadata;
use std::collections::HashMap;
use std::time::Instant;
mod file_searchers;
use file_searchers::{display_results, file_search};
mod cache_file_index;
use cache_file_index::create_and_cache_file_index;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // create_b_index();
    // let file_index = file_indexers::create_hash_index();
    let file_index = file_indexers::create_hash_index_parallel();
    create_and_cache_file_index()?;
    println!("File Cache Created!");

    // count the number of files in the index
    println!("Number of files indexed: {}", file_index.len());

    // println!("{:?}", file_index[".././web_assembly/game-of-life/www/node_modules/es-abstract/2016/OrdinaryGetOwnProperty.js"]);
    println!("Indexing in Progress...");
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());

    println!("Searching for files...");
    let start = Instant::now();
    let search_result = file_search("OrdinaryGetOwnProperty", &file_index);
    display_results(search_result);
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());

    Ok(())
}
