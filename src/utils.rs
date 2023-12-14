use std::io::{self, Read};


// # Arguments
// * `question` - A string slice that holds the question to ask the user
// # Returns returns a string slice that holds the answer to the question
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

