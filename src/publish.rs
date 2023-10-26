use std::io::{self, Read};
use std::borrow::Cow;
use std::fs::{self};
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

// This function pushes to a remote there must be a remote already in the repository
// gitcli = git push
// # Arguments: the repository mut git2::Repository, the remote git2::Remote, the remote url string slice
// # Returns: nothing
fn push_to_remote(repo: &Repository, _remote_url: &str) {
    print!("Push to remote");
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

pub fn commit_changes(repo: &Repository, parent_commit: Option<&git2::Commit>) {
    print!("Commit changes");
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
    print!("\nInitial commit");

    // Create an index adding all files to staging area
    let mut index = repo.index().expect("Failed to open index");
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .expect("Failed to add to index");
    index.write().expect("Failed to write index");
    // Create a tree from the index
    let tree_id = index.write_tree().expect("Failed to write tree");

    // Get the committer's information
    let signature = Signature::now(&get_username()
        .expect("Failed to get username"), &get_user_email()
        .expect("Failed to get email")).expect("Failed to create signature");
    
    print!("\nCommiting Repo" );

    let _commit_oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &repo.find_tree(tree_id).expect("Failed to find tree"),
        &[],  // Provide as a slice
    ).expect("Failed to commit");

    println!("Committed to main branch with commit id: {}", &_commit_oid)
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
pub fn prompt_yes_no(question: &str) -> bool {
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
    //get username
    let mut command = Command::new("git");
    command.arg("config");
    command.arg("user.name");
    let out = command.output().expect("Failed to execute command");
    //return username;
    let username = str::from_utf8(&out.stdout).ok()?.trim().to_string();
    Some(username)
}
// This function gets the email of the user
// # Return string slice that holds the email
fn get_user_email() -> Option<String> {
    //get email
    let mut command = Command::new("git");
    command.arg("config");
    command.arg("user.email");
    let out = command.output().expect("Failed to execute command");
    //return email;
    let email = str::from_utf8(&out.stdout).ok()?.trim().to_string();
    Some(email)
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


