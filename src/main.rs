use std::fs::File;
use std::path::PathBuf;
use std::{collections::BTreeMap, collections::HashMap, time::SystemTime};

// use std::fs::Metadata;s
use std::time::{Duration, Instant};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // create_b_index();
    create_hash_index();
    println!("Indexing in Progress...");
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
    Ok(())
}
#[derive(Debug)]
struct FileMetadata {
    name: String,
    size: u64,
    created: SystemTime,
    modified: SystemTime,
    accessed: SystemTime,
}
type Path = String;
fn create_b_index() -> BTreeMap<PathBuf, FileMetadata> {
    let mut b_index: BTreeMap<PathBuf, FileMetadata> = BTreeMap::new();

    for entry in WalkDir::new("../.").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();

            let metadata = entry.metadata().unwrap();
            let accessed = metadata
                .accessed()
                .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
            let created = metadata
                .created()
                .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
            let modified = metadata
                .modified()
                .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
            b_index.insert(
                path.clone(),
                FileMetadata {
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: metadata.len(),
                    created,
                    modified,
                    accessed,
                },
            );
        }
        // println!("{:?}", entry);
    }

    b_index
}
fn create_hash_index() -> HashMap<String, FileMetadata> {
    let mut files: HashMap<Path, FileMetadata> = HashMap::new();
    for entry in WalkDir::new("../.").into_iter().filter_map(|e| e.ok()) {
        // make sure we only get files
        if entry.file_type().is_file() {
            let path = entry.path().to_str().unwrap();

            //@note metadata is a method provided by `walkdir::DirEntry`
            // @note also possible with fsmetadata(filename)
            let metadata = entry.metadata().unwrap();
            let accessed = metadata
                .accessed()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            let created = metadata
                .created()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            let modified = metadata
                .modified()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            files.insert(
                path.to_string(),
                FileMetadata {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    // @todo update this value
                    size: metadata.len(),
                    created: created,
                    modified: modified,
                    accessed: accessed,
                },
            );
        }
    }
    files
}
