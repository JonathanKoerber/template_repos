use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn extract_two_digit_numbers(s: &str) -> Vec<u32> {
    let mut numbers = Vec::new();

    for i in 0..s.len() - 1 {
        if let Ok(num) = s[i..i + 2].parse::<u32>() {
            if num >= 1 && num <= 10 {
                numbers.push(num);
            }
        }
    }

    numbers
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

    // Create a map of two-digit numbers to directory names
    let mut dir_map: HashMap<u32, String> = HashMap::new();

    for dir in directories {
        let numbers = extract_two_digit_numbers(&dir);
        for num in numbers {
            dir_map.insert(num, dir.clone());
        }
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

    // Move files to respective directories based on the extracted two-digit numbers
    for file in files {
        if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
            let numbers = extract_two_digit_numbers(file_name);
            for num in numbers {
                if let Some(dir_name) = dir_map.get(&num) {
                    let dest_path = Path::new(&dir_name).join(file.file_name().unwrap_or_default());
                    print!("Moving {} to: {}", file_name, &dest_path.display());
                    fs::rename(&file, dest_path).expect("Failed to move file");
                    print!("Files have been moved check the assignment directorys.")
                }
            }
        }
    }
}
