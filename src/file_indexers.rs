use crate::file_metadata::FileMetadata;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

pub fn create_hash_index() -> HashMap<String, FileMetadata> {
    let mut files: HashMap<String, FileMetadata> = HashMap::new();

    for entry in WalkDir::new("../..").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_str().unwrap();
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

            files.insert(
                path.to_string(),
                FileMetadata {
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: metadata.len(),
                    created,
                    modified,
                    accessed,
                },
            );
        }
    }

    files
}
pub fn create_b_index() -> BTreeMap<PathBuf, FileMetadata> {
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

// @note almost twice as fast as the sequential version, especially on large directories
// @todo optimize further
pub fn create_hash_index_parallel() -> HashMap<String, FileMetadata> {
    let files: HashMap<String, FileMetadata> = WalkDir::new("../")
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge() // Convert iterator to a parallel iterator
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let path = entry.path().to_str().unwrap();
            let metadata = entry.metadata().unwrap();
            let accessed = metadata.accessed().unwrap_or(SystemTime::UNIX_EPOCH);
            let created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
            let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

            (
                path.to_string(),
                FileMetadata {
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: metadata.len(),
                    created,
                    modified,
                    accessed,
                },
            )
        })
        .collect();

    files
}
