use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "../../../vuejs";
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
    println!("Hello, world!");

    Ok(())
}
