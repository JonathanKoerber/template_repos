use std::fs;
use std::io::{self};

use git2::{Repository, RepositoryInitOptions};
use crate::git_commands::initial_commit;
use crate::utils::prompt_yes_no;

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
                        let prompt_question = format!("Would you like to create a Git repository in this directory {}?", dir_path.display());
                        if prompt_yes_no(&prompt_question){
                            // Create a Git repository with main as the default branch
                            let mut init_opts = RepositoryInitOptions::new();
                            init_opts.initial_head("refs/heads/main");
                            match Repository::init_opts(&dir_path, &init_opts) {
                                Ok(_) => println!("Git repository created successfully in '{}'", dir_path.display()),
                                Err(err) => eprintln!("Failed to create Git repository: {}", err),
                            }
                         if  prompt_yes_no("Would you like to make an initial commit?"){
                             // Get the repository
                             if let Ok(repo) = Repository::open(&dir_path) {
                                 // Commit the changes
                                 initial_commit(&repo);
                                 // print remote url to screen
                             }
                         }
                     }
                    }
                }
            }
        }
    }
}

