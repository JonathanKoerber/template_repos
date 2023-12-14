use crate::git_credentials::get_user_email;
use crate::git_credentials::get_username;
use crate::git_credentials::create_remote_callbacks;
use git2::{Repository, Status, Remote, Signature, PushOptions, RemoteCallbacks};


// This function pushes to a remote there must be a remote already in the repository
// git cli = git push
// # Arguments: the repository mut git2::Repository, the remote git2::Remote, the remote url string slice
// # Returns: nothing
pub fn push_to_remote(repo: &Repository, _remote_url: &str) {
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

// Create a commit with in a directory is there not a commit HEAD
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

    // Get the collect signature for commit information
    let signature = Signature::now(&get_username()
        .expect("Failed to get username"), &get_user_email()
        .expect("Failed to get email")).expect("Failed to create signature");

    print!("\nCommitting Repo" );

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

pub fn has_uncommitted_changes(repo: &Repository) -> bool {
    let statuses = repo.statuses(None).expect("Failed to get statuses");
    for entry in statuses.iter() {
        if entry.status() != Status::CURRENT {
            return true;
        }
    }
    false
}
