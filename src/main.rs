use std::env;
use dotenv::dotenv;
use std::path::Path;
mod build_dir;
mod match_files;
mod publish;
mod create_repo;
mod unzip;
use std::io::{self, Read};
use std::fs::{self};

fn read_public_key(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    dotenv().ok();

    // let public_key_path = "/home/jk/.ssh/id_ed25519.pub";
    // println!("public_key_path: {}", public_key_path);
    // println!("public key: {}", Path::new(&public_key_path).display());
    // match read_public_key(public_key_path) {
    //     Ok(contents) => println!("Public key contents: {}", contents),
    //     Err(err) => eprintln!("Failed to read public key: {}", err),
    // }
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rust_cli <command> [args]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "build-dir" => {
            if args.len() != 4 {
                println!("Usage: rust_cli build-dir <template-string> <number-of-dirs>");
                return;
            }
            let template_string = &args[2];
            let num_dirs = args[3].parse::<u32>().expect("Invalid number of directories");
            build_dir::build_dir(template_string, num_dirs);
        }
        "build-README" => {
            if args.len() != 7 {
                
                println!("args: {:?}", args);
                println!("Usage: rust_cli build-README <template-string> <number-of-dirs> <course-name> <course-type> <readme-path>");
                return;
            }
            println!("build-README");
            println!("args: {:?}", args);
            let template_string = &args[2];
            let num_dirs = args[3].parse::<u32>().expect("Invalid number of directories");
            let course_name = &args[4];
            let course_type = &args[5];
            let readme_path = &args[6];
            build_dir::build_dir_readme(template_string, num_dirs, course_name, course_type, readme_path);
        }
        "match-files" => {
            match_files::match_files();
        }
        "unzip" => {
            if args.len() != 3 {
                println!("Usage: rust_cli unzip <dir of ziped file>");
                return;
            }
            let dest = &args[2];
            unzip::unzip(dest);
        }
        "publish" => {
            if args.len() != 3 {
                println!("Usage: rust_cli publish <organization>");
                return;
            }
            let org = &args[2];
            publish::publish(org);
        }
        "create_repo" => {
            create_repo::create_repo();
        }
        _ => {
            println!("Unknown command. Available commands: build-dir, match-files, publish");
        }
    }
}