use std::fs;
use std::io::{self};
use git2::{Repository, RepositoryInitOptions};
use crate::publish::initial_commit;

pub fn create_repo() {
    // Specify the directory to search for repositories
    let directory_to_search = ".";  // Change this to your desired directory

    // Get a list of subdirectories
    if let Ok(entries) = fs::read_dir(directory_to_search) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let dir_path = entry.path();
                    let repo_path = dir_path.join(".git");

                    // Check if the directory is a Git repository
                    if !repo_path.exists() {
                        println!("Directory '{}' does not have a Git repository.", dir_path.display());

                        // Ask the user if they want to create a Git repository
                        println!("Would you like to create a Git repository in this directory? (y/n)");

                        let mut response = String::new();
                        io::stdin().read_line(&mut response).expect("Failed to read user input");

                        if response.trim().to_lowercase() == "y" {
                            // Create a Git repository
                            match Repository::init_opts(&dir_path, &RepositoryInitOptions::new()) {
                                Ok(_) => println!("Git repository created successfully in '{}'", dir_path.display()),
                                Err(err) => eprintln!("Failed to create Git repository: {}", err),
                            }
                        }
                        println!("Would you like to make an initial commit? (y/n)");
                        let mut response = String::new();
                        io::stdin().read_line(&mut response).expect("Failed to read user input");
                        if  response.trim().to_lowercase() == "y" {
                            // Get the repository
                            if let Ok(repo) = Repository::open(&dir_path) {
                                // Commit the changes
                                initial_commit(&repo);
                            }
                        }
                    }
                }
            }
        }
    }
}

