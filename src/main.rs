mod file_indexers;
mod file_metadata;
use std::time::Instant;
mod file_searchers;
use file_searchers::{display_results, file_search};
mod cache_file_index;
use cache_file_index::create_and_cache_file_index_bin;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Search {
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    file: bool,
    #[clap(short, long)]
    directory: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = Search::parse();
    println!("{:?}", query);
    let file_to_search = query.name;

    let start = Instant::now();
    let file_index = create_and_cache_file_index_bin()?;

    // println!("{:?}", file_index[".././web_assembly/game-of-life/www/node_modules/es-abstract/2016/OrdinaryGetOwnProperty.js"]);
    println!("Indexing in Progress...");
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
    // count the number of files in the index
    println!("Number of files indexed: {}", file_index.len());

    println!("Searching for files...");
    let start = Instant::now();
    let search_result = file_search(file_to_search.as_str(), &file_index);
    display_results(search_result);
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_secs_f64());

    Ok(())
}
