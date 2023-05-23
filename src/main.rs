use std::{collections::HashMap, time::SystemTime};
// use std::fs::Metadata;s
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    struct FileMetadata {
        name: String,
        size: u64,
        created: SystemTime,
        modified: SystemTime,
        accessed: SystemTime,
    }

    // @todo update to use PathBuf
    type Path = String;

    // create a hash map of files to their metadata
    let mut files: HashMap<Path, FileMetadata> = HashMap::new();

    let mut path = "../../../vuejs";
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        // make sure we only get files
        if entry.file_type().is_file() {
            path = entry.path().to_str().unwrap();
            //@note metadata is a method provided by `walkdir::DirEntry`
            // @note also possible with fsmetadata(filename)
            let metadata = entry.metadata()?;
            let accessed = metadata
                .accessed()
                .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            // @todo update this value
            let size = metadata.len();
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
                    size: metadata.len(),
                    created: created,
                    modified: modified,
                    accessed: accessed,
                },
            );
        }
    }

    Ok(())
}
