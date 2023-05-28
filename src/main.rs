use std::fs::File;
use std::path::PathBuf;
use std::{collections::BTreeMap, collections::HashMap, time::SystemTime};
mod file_indexers;

// use std::fs::Metadata;s
use std::time::{Duration, Instant};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // create_b_index();
    // let file_index = file_indexers::create_hash_index();
    let file_index = file_indexers::create_hash_index_parallel();
    // println!("{:?}", file_index[".././web_assembly/game-of-life/www/node_modules/es-abstract/2016/OrdinaryGetOwnProperty.js"]);
    println!("Indexing in Progress...");
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
    Ok(())
}
