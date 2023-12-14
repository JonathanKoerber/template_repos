use crate::utils::prompt_yes_no;
use crate::git_commands::commit_changes;
use crate::git_commands::push_to_remote;
use crate::git_commands::has_uncommitted_changes;
use std::path::{Path};
use std::process::Command;
use std::{env, str, error::Error};
use dotenv::dotenv;
use git2::{Repository, Status, Signature, Remote, RemoteCallbacks, Cred, PushOptions, BranchType};

// Publish Function:
// The publish function is called, and it checks if a repository exists in a directory.
// If a repository exists, it checks for uncommitted changes. If there are uncommitted changes, it calls handle_repo_changes. If not, it calls handle_no_remote.

// handle_repo_changes:
// This function handles the case where there are uncommitted changes in the repository.
// It tries to get the parent commit, and if successful, it calls commit_changes or performs other actions.

// handle_no_remote:
// This function handles the case where there is no remote in the repository.
// It prompts the user to add a remote. If the user wants to add a remote, it calls push_to_remotes to set up the remote and push to it.
//
// # Arguments: the organization string slice
// # Returns: nothing

pub fn publish(org_string: &str) {
    dotenv().ok();
    let directory_to_search = ".";
    if let Ok(entries) = std::fs::read_dir(directory_to_search) {
        
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let dir_path = entry.path();
                    let repo_path = dir_path.join(".git");
                    if repo_path.exists() {
                        print!("Found repository: {:?} ", dir_path.display());
                        // Open the repository
                        if let Ok(repo) = Repository::open(&dir_path) {
                            let has_uncommitted_changes = has_uncommitted_changes(&repo);
                            if has_uncommitted_changes {
                               // Todo Refactor to method call handle_repo_changes
                                if let Ok(reference) = repo.head() {
                                    if let Ok(parent_commit) = reference.peel_to_commit() {
                                        // Obtain a mutable reference to the remote
                                        if let Ok(mut remote) = repo.find_remote("origin") {
                                            // Call handle_repo_changes with the mutable remote
                                            handle_repo_changes(&repo, Some(&parent_commit), Some(&mut remote));
                                        } else {
                                            panic!("Remote 'origin' not found");
                                        }
                                    } else {
                                        panic!("Failed to get parent commit");
                                    }
                                } else {
                                    print!("Failed to get HEAD reference");
                                    handle_no_remote(&repo, org_string, &dir_path);
                                   // panic!("Failed to get HEAD reference");

                                }
                            } else {
                                handle_no_remote(&repo, org_string, &dir_path);
                            }
                        } 
                    }
                }
            }else{
                println!("No repository were found try funning create_repo first");
            }
        }
    }
}

//This function handles the case where there are uncommitted changes in the repository
// git cli = git add .  
// git cli = git commit -m "Adding Content"
// # Arguments: the repository git2::Repository, the parent commit git2::Commit
// # Returns: nothing
fn handle_repo_changes(repo: &Repository, parent_commit: Option<&git2::Commit>, remote: Option<&mut Remote>) {
    println!("Uncommitted changes found.");
    println!("handle_repo_changes");
    let head = repo.head().expect("Failed to get HEAD");
    if head.target().is_none() {
        println!("No HEAD found. Perform initial commit.");
        panic!("Run create_repo first");
    } else {
        println!("Uncommitted changes found.");
        if prompt_yes_no("Would you like to commit these changes?") {
            commit_changes(repo, parent_commit);
        }else{
            panic!("Unexpected error");
        }
    if prompt_yes_no("Would you like to push these changes?") {
        if let Some(current) = remote {
            if let Some(url) = current.url() {
                println!("Remote URL: {}", url);
                push_to_remote(repo, url);
            } else {
                println!("Failed to retrieve remote URL.");
                return;
            }
        } else {
            panic!("No valid remote provided.");
        }
            
    }
}
}
//This function handles the case where there is no remote in the repository

// git cli = git remote add origin git@git:com:org_string/dir_path
// # Arguments: the repository git2::Repository, the organization string slice, the directory path
// # Returns: nothing
fn handle_no_remote(_repo: &Repository, org_string: &str, dir_path: &Path) {

    print!("\nNo remote found\n");
    if prompt_yes_no("Would you like to add a remote?") {
        let remote_name = dir_path
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert to string");

        let remote_url = format!("git@github.com:{}/{}.git", org_string, remote_name);
        println!("Remote: {} look good?", remote_url);

        if prompt_yes_no("Do you like this remote address? ") {
            let remote_url = remote_url.trim();
            let mut add_origin = Command::new("git");
            add_origin.arg("remote");
            add_origin.arg("add");
            add_origin.arg("origin");
            print!("remote_url: {}", remote_url);
            add_origin.arg(remote_url);
           
            let out = add_origin.output().expect("Failed to execute command");
            println!("status: {}", out.status);

            // Print the standard output
            let out_str = str::from_utf8(&out.stdout).expect("Failed to convert stdout to string");
            println!("Standard output: {}", out_str);
            // Print the standard error
            let err_str = str::from_utf8(&out.stderr).expect("Failed to convert stderr to string");
            println!("Standard error: {}", err_str);
        } else {
            println!("No remote added");
        }
    }
    let has_uncommitted_changes = has_uncommitted_changes(_repo);
    if has_uncommitted_changes {
        if prompt_yes_no("You have uncommited changes. Would you like to commit them?") {
            let head_ref = _repo.head().expect("Failed to get HEAD");
                if let Ok(parrent_commit) = head_ref.peel_to_commit(){
            commit_changes(_repo, Some(&parrent_commit));
                } else {
                    panic!("Failed to get parent commit");
                }
        }
    }
    // git remote -u origin main
    if prompt_yes_no("Would you like to try and create the remote?"){
        let mut create_remote = Command::new("git");
        create_remote.arg("");
        create_remote.arg("-u");
        create_remote.arg("origin");
        create_remote.arg("main");
        let out = create_remote.output().expect("Failed to execute command");
        println!("status: {}", out.status);
    }
}





