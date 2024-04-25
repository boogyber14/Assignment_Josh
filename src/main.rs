extern crate dirs;
extern crate walkdir;
extern crate inquire;

use walkdir::WalkDir;
use inquire::Select;

fn main() {
    // Get user's home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            println!("Unable to get home directory.");
            return;
        }
    };

    // Walk through the user's home directory and collect subdirectories
    let mut subdirs = Vec::new();
    for entry in WalkDir::new(&home_dir).min_depth(1).max_depth(1) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.file_type().is_dir() {
            subdirs.push(entry.path().to_path_buf());
        }
    }

    // Convert subdirectories to strings
    let subdir_names: Vec<String> = subdirs.iter()
        .filter_map(|path| path.file_name().and_then(|name| name.to_str().map(String::from)))
        .collect();

    // Prompt user to select a directory
    let selected_dir = Select::new("Select a directory:", subdir_names)
        .prompt();

    // Print selected directory or exit
    match selected_dir {
        Ok(selected) => {
            println!("Selected directory: {:?}", selected);
            // Now you can proceed with further operations within the selected directory
        }
        Err(error) => println!("Error: {}", error),
    }
}
