
use std::fs::{self, File};
use std::io::{Write};
use std::path::{Path};

pub fn build_dir(template_string: &str, num_dirs: u32) {
    for i in 1..=num_dirs {
        let dir_name = format!("{}{:02}", template_string, i);
        let dir_path = Path::new(".").join(&dir_name);

        fs::create_dir(&dir_path).expect("Failed to create directory");

        println!("Created directory: {:?}", dir_path);
    }
}


pub fn build_dir_readme(template_string: &str, num_dirs: u32, course_name: &str, course_type: &str, readme_path: &str) {
    for i in 1..=num_dirs {
        let dir_name = format!("{}{:02}", template_string, i);
        let dir_path = Path::new(".").join(&dir_name);

        fs::create_dir(&dir_path).expect("Failed to create directory");

        println!("Created directory: {:?}", dir_path);

        // Read template README content
        let template_readme_content = read_template_readme_content(); 
        
        // Create README content
        let readme_content = format!(
            "# {}\n\n### {} {:02}{}",
            course_name, course_type, i, template_readme_content
        );

        // Write README content to file
        let mut readme_file = File::create(dir_path.join(readme_path)).expect("Failed to create README file");
        writeln!(readme_file, "{}", readme_content).expect("Failed to write README content");
    }
}

fn read_template_readme_content() -> String {
    // Assuming you have a template README file in the same directory as your executable
    let template_readme_path = Path::new("README.md");
    match fs::read_to_string(template_readme_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading template README: {}", err);
            String::new()
        }, // Return an empty string if there's an error reading the template file
    }
}


// pub fn build_dir_readme(template_string: &str, num_dirs: u32, course_name: &str, course_type: &str, readme_path: &str) {
//     for i in 1..=num_dirs {
//         let dir_name = format!("{}{:02}", template_string, i);
//         let dir_path = Path::new(".").join(&dir_name);

//         fs::create_dir(&dir_path).expect("Failed to create directory");

//         println!("Created directory: {:?}", dir_path);

//         // Create README content
//         let readme_content = format!(
//             "# {}\n\n### {} {:02}\n\n",
//             course_name, course_type, i
//         );

//         // Append content to README file
//         let mut readme_file = fs::OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(dir_path.join(readme_path))
//         .expect("Failed to open README file");

// // Append content to README file
// writeln!(readme_file, "{}", readme_content)
//     .expect("Failed to write README content");
// }
// }

