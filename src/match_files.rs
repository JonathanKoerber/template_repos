use std::fs;
use std::path::{Path};
use std::collections::HashMap;

fn get_last_four_chars(s: &str) -> String {
    if s.len() <= 4 {
        s.to_string()
    } else {
        s[s.len() - 4..].to_string()
    }
}
fn get_first_four_chars(s: &str) -> String {
    if s.len() <= 4 {
        s.to_string()
    } else {
        s[..4].to_string()
    }
}
pub fn match_files() {
    // Get all directories in the current directory
    let directories: Vec<String> = fs::read_dir(".")
    .expect("Failed to read directory")
    .filter_map(|entry| {
        if let Ok(entry) = entry {
            if entry.file_type().ok()?.is_dir() {
                return Some(entry.file_name().to_string_lossy().into_owned());
            }
        }
        None
    })
    .collect();


    // Create a map of last four characters of directory names to directory names
    let mut dir_map: HashMap<String, String> = HashMap::new();

    for dir in directories {
        let last_four = get_last_four_chars(&dir);
        dir_map.insert(last_four, dir);
    }

    // Get all files in the current directory
    let files = fs::read_dir(".")
        .expect("Failed to read directory")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().ok()?.is_file() {
                    return Some(entry.path());
                }
            }
            None
        });

    // Move files to respective directories based on the last four characters of their names
    for file in files {
        if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
            let first_four = get_first_four_chars(file_name);
            if let Some(dir_name) = dir_map.get(&first_four) {
                let dest_path = Path::new(&dir_name).join(file.file_name().unwrap_or_default());
                fs::rename(&file, dest_path).expect("Failed to move file");
            }
        }
    }
}
