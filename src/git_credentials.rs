use std::io::{self, Read};
use std::borrow::Cow;
use std::fs::{self};
use std::{env, str, error::Error};
use git2::{RemoteCallbacks, Cred};
use std::process::Command;
use std::path::{Path};

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
// This function gets the username the user is logged in
// # Return string slice that holds the username
pub fn get_username() -> Option<String> {
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
pub fn get_user_email() -> Option<String> {
    //get email
    let mut command = Command::new("git");
    command.arg("config");
    command.arg("user.email");
    let out = command.output().expect("Failed to execute command");
    //return email;
    let email = str::from_utf8(&out.stdout).ok()?.trim().to_string();
    Some(email)
}

// This function creates a ssh git2 Cred object
// GitHub keypath neex to be set in .env file
// # Arguments: the username string slice
// # Return a Cred object
pub fn create_remote_callbacks<'a>() -> RemoteCallbacks<'a> {
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
