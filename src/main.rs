mod file_indexers;
mod file_metadata;
use std::time::Instant;
mod file_searchers;
use file_searchers::{display_results, file_search_parallel};
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

    println!("Indexing in Progress...");
    let duration = start.elapsed();
    let time_taken_for_indexing = duration.as_secs_f64();
    let num_file_indexed = file_index.len();

    println!("Searching for files...");
    let file_search_start = Instant::now();
    let search_result = file_search_parallel(file_to_search.as_str(), &file_index);
    let files_found = search_result.len();
    display_results(search_result);
    let duration = file_search_start.elapsed();
    let time_taken_for_file_search = duration.as_secs_f64();

    // save benchmark results to a file
    std::fs::write(
        "benchmark.txt",
        format!(
            "Time taken for indexing: {} seconds\nNumber of files indexed: {}\nTime taken for searching: {} seconds\n Files found: {}\n",
            time_taken_for_indexing,
            num_file_indexed,
            time_taken_for_file_search,
            files_found
        ),
    )?;

    Ok(())
}
