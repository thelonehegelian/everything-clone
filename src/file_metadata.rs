use std::time::SystemTime;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub accessed: SystemTime,
}
