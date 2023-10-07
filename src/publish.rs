use std::io::{self, Read};
use std::borrow::Cow;
use std::fs::{self};
use std::path::{Path};
use std::process::Command;
use std::{env, str, error::Error};
use dotenv::dotenv;
use git2::{Repository, Status, Signature, Remote, RemoteCallbacks, Cred, PushOptions};

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
                                    panic!("Failed to get HEAD reference");
                                }
                            } else {
                                handle_no_remote(&repo, org_string, &dir_path);
                            }
                        } else {
                            println!("Failed to open repository for {:?}", dir_path);
                        }
                    }
                }
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
    println!("No remote found");

    if prompt_yes_no("Would you like to add a remote?") {
        let remote_name = dir_path
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert to string");

        let remote_url = format!("git@git.com:{}/{}", org_string, remote_name);
        println!("Remote: {} look good?", remote_url);

        if prompt_yes_no("Would you like to add a remote? ") {
            let remote_url = remote_url.trim();
            let mut add_origin = Command::new("git");
            add_origin.arg("remote");
            add_origin.arg("add");
            add_origin.arg("origin");
            add_origin.arg(remote_url);
            let out = add_origin.output().expect("Failed to execute command");
            println!("status: {}", out.status);
            println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
            // if let Ok(mut remote) = repo.remote_anonymous(remote_url) {

            //     push_to_remote(repo, remote_url);
            // } else {
            //     println!("Failed to create remote");
            // }
        } else {
            println!("No remote added");
        }
    }
    let has_uncommitted_changes = has_uncommitted_changes(_repo);
    if has_uncommitted_changes {
        if prompt_yes_no("Would you like to commit these changes?") {
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

// This function pushes to a remote there must be a remote already in the repository
// gitcli = git push
// # Arguments: the repository mut git2::Repository, the remote git2::Remote, the remote url string slice
// # Returns: nothing
fn push_to_remote(repo: &Repository, _remote_url: &str) {
    let mut remote = repo.find_remote("origin").expect("Failed to find remote");
    let _remote_callbacks = RemoteCallbacks::new();
    // Set up your remote callbacks as needed
    let mut options = PushOptions::new();
    options.remote_callbacks(create_remote_callbacks());
    println!("Pushing to remote...");
    // Push to the remote with the given URL
    
    remote
        .push(&[String::from("refs/heads/main:refs/heads/main")], Some(&mut options))
        .expect("Failed to push");
}

// fn run_command(command: &str) {
//     let output = Command::new("bash")
//         .arg("-c")
//         .arg(command)
//         .output()
//         .expect("Failed to execute command");

//     if !output.status.success() {
//         eprintln!("Command failed: {:?}", command);
//         eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
//     }
// }
// fn push_to_remotes(repo: &Repository, maybe_remote: Option<&mut Remote>) {
//     if let Some(mut remote) = maybe_remote.cloned() {
//         let mut remote_callbacks = RemoteCallbacks::new();
//         // Set up your remote callbacks as needed
//         let mut options = PushOptions::new();
//         options.remote_callbacks(create_remote_callbacks());
//         println!("Pushing to remote...");
//         // Push to the remote with the given URL
        
//         remote
//             .push(&[String::from("refs/heads/main:refs/heads/main")], Some(&mut options))
//             .expect("Failed to push");
//     } else {
//         println!("No valid remote provided.");
//     }
// }


//This function commit changes to a repository that has a commit HEAD
// # Arguments: the repository git2::Repository, the parent commit git2::Commit
// # Returns: nothing

pub fn commit_changes(repo: &Repository, parent_commit: Option<&git2::Commit>) {
    // Create an index to prepare for the commit
    let mut index = repo.index().expect("Failed to open index");
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).expect("Failed to add to index");
    index.write().expect("Failed to write index");

    // Create a tree from the index
    let tree_id = index.write_tree().expect("Failed to write tree");

    // Get the HEAD reference
    let _head = repo.head().expect("Failed to get HEAD");

    // Get the committer's information
    let signature = Signature::now(&get_username().expect("Failed to get username"), &get_user_email().expect("Failed to get email")).expect("Failed to create signature");
    
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Adding Content",
        &repo.find_tree(tree_id).expect("Failed to find tree"),
        &[parent_commit.unwrap()],  // Provide as a slice
    ).expect("Failed to commit");
    
}
// Create a commit with in a dirtory is there not a commit HEAS
// # Arguments: the repository git2::Repository
// # Returns: nothing
pub fn initial_commit(repo: &Repository) {
    // Create an index to prepare for the commit
    let mut index = repo.index().expect("Failed to open index");
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).expect("Failed to add to index");
    index.write().expect("Failed to write index");

    // Create a tree from the index
    let tree_id = index.write_tree().expect("Failed to write tree");

    // Get the committer's information
    let signature = Signature::now(&get_username().expect("Failed to get username"), &get_user_email().expect("Failed to get email")).expect("Failed to create signature");
    
    let _commit_oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &repo.find_tree(tree_id).expect("Failed to find tree"),
        &[],
    ).expect("Failed to commit");
}
fn has_uncommitted_changes(repo: &Repository) -> bool {
    let statuses = repo.statuses(None).expect("Failed to get statuses");
    for entry in statuses.iter() {
        if entry.status() != Status::CURRENT {
            return true;
        }
    }
    false
}
// # Arguments
// * `question` - A string slice that holds the question to ask the user
// # Returns retruns a string slice that holds the answer to the question
fn prompt_yes_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read user input");
    if response.trim().to_lowercase() == "y"{
        true
    } else {
        false
    }
}
// This function gets the username the user is logged in 
// # Return string slice that holds the username
fn get_username() -> Option<String> {
    if cfg!(target_os = "windows") {
        Some(env!("USERNAME").to_string())
    } else {
        let output = Command::new("whoami").output().ok()?;
        let username = str::from_utf8(&output.stdout).ok()?.trim();
        Some(username.to_string())
    }
}
// This function gets the email of the user
// # Return string slice that holds the email
fn get_user_email() -> Option<String> {
    if cfg!(target_os = "windows") {
        // Windows specific code to get email
        // Replace with the appropriate method to retrieve the email on Windows
        Some("windows@example.com".to_string())
    } else {
        // For Unix-like systems, you might use a command to get the email
        let output = Command::new("sh")
            .arg("-c")
            .arg("echo $USER@$HOSTNAME")
            .output()
            .ok()?;
        let email = str::from_utf8(&output.stdout).ok()?.trim();
        Some(email.to_string())
    }
}

// This function reads the public key from the path
// # Arguments: the path string slice
// # Returns: a io::Result<String>
// yay
fn read_public_key(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// This function creates a ssh git2 Cred object
// GitHub keypath neex to be set in .env file
// # Arguments: the username string slice
// # Return a Cred object
fn create_remote_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        match create_ssh_cred() {
            Ok(cred) => Ok(cred),
            Err(_) => Err(git2::Error::from_str("Failed to create SSH credentials")),
        }
    });
    callbacks
}



fn create_ssh_cred() -> Result<Cred, Box<dyn Error>> {
    let public_key_path = "/home/jk/.ssh/id_ed25519.pub";
    //let public_key_path = env::var("PUBLIC_KEY_PATH").expect("Failed to get public key path");
    // let private_key_path = env::var("PRIVATE_KEY_PATH").expect("Failed to get private key path");

    let _pub_key: Option<Cow<'_, [u8]>> = Some(fs::read(&public_key_path).unwrap().into());
    let pub_key_path = Path::new(&public_key_path);
    //panic!("public key path: {}", pub_key_path.display());
    //let private_key: Option<Cow<'_, [u8]>> = Some(fs::read(&private_key_path).unwrap().into());
    let username = get_username().expect("Failed to get username");
    Ok(Cred::ssh_key(
        &username,  // Provide the username
        None,               // Use default username if None provided
        &pub_key_path,
        None,
    )?)
}


