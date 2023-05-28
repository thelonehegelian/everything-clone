mod file_indexers;
mod file_metadata;
use file_metadata::FileMetadata;
use std::collections::HashMap;
use std::time::Instant;
mod file_searchers;
use file_searchers::file_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // create_b_index();
    // let file_index = file_indexers::create_hash_index();
    let file_index = file_indexers::create_hash_index_parallel();
    // print the file index
    println!("{:?}", file_index);
    // println!("{:?}", file_index[".././web_assembly/game-of-life/www/node_modules/es-abstract/2016/OrdinaryGetOwnProperty.js"]);
    println!("Indexing in Progress...");
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());

    println!("Searching for files...");
    let start = Instant::now();
    file_search("OrdinaryGetOwnProperty", &file_index);
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());

    Ok(())
}
