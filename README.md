## CLI Tool for Repository Management

This CLI tool provides functionality for repository management and organization. It includes the following commands:

- `build-dir`: Build a directory structure based on a given template string and the number of directories.
- `build-README`: Build a README file based on a template string, the number of directories, course name, course type, and README path.
- `match-files`: Match and process files in the directory.
- `publish`: Publish a repository for an organization.
- `create_repo`: Create a new repository.

### Getting Started

This section provides a guide on how to get started with the CLI tool for repository management. I created this to build a lot of template repositorys for GitHub Classrooms. 

Prerequisites

Before using the CLI tool, ensure you have the following prerequisites installed on your system:

Rust Programming Language: Ensure Rust is installed on your system. If not, you can download and install Rust from the official Rust website.


### Installation

To install the CLI tool, follow these steps:
    
Clone the repository containing the CLI tool to your local machine:

```
git clone <repository-url>    
```

Change to the project directory:

```
cd <project-directory>
```

### Build the Rust project using Cargo:


```
cargo build --release
```

### Usage

To use the CLI tool, run the executable with the desired command and required arguments. Here are the available commands and their respective usage:

1. **Build Directory (`build-dir`)**:
   ```
   rust_cli build-dir <template-string> <number-of-dirs>
   ```
    * this will create an manny directry as you chould want in the for template-string01...template-string\<number-of-dirs>. I usally just need to make 10. So there are just 2 didgets. 
2. **Build with README.md template (`build-README`)**:
   ```
   rust_cli build-README <template-string> <number-of-dirs> <course-name> <course-type> <readme-filename>
   ```
    * This dose the same thing as build-dir but templates README.md file in the root dir. 
    * This will look for a file named \<readme-filename> the add the \<course-name> \\n then \<course-type> and like the preivous command module number <number-of-dirs> to each directory.
3. **Match Files (`match-files`)**:
   ```
   rust_cli match-files
   ```
    * If you have already run either of the build dirs command and have a buch course module it will sort them into the matching dir
5. **Create Repository (`create_repo`)**:
   ```
   rust_cli create_repo
   ```
    * this will got through all the dirs for build-dir if there in no .git it will intilize the repo. 
    
4. **Publish Repository (`publish`)**:
   ```
   rust_cli publish <organization>
   ```
   this doesn't work right now ther is an issue with credentails to push to github
   * This pushes all uncomitted work to githup. 
   * If there is no remote it will create the remote repo.
   * yeha



### Example Usage

1. Build a directory:
   ```
   rust_cli build-dir "template-string" 10
   ```

2. Build README for a course:
   ```
   rust_cli build-README "template-string" 5 "Course Name" "Course Type" "/path/to/readme.md"
   ```

3. Match files in the current directory:
   ```
   rust_cli match-files
   ```
4. Create a new repository:
   ```
   rust_cli create_repo
   ```

5. Publish a repository for an organization:
   ```
   rust_cli publish "organization-name"
   ```


### Workflow

The logical workflow involves the following modules:

- `mod build_dir`: Handles building a directory structure based on a template string and the number of directories.
- `mod match_files`: Handles matching and processing files in a directory.
- `mod create_repo`: Handles creating a new repository.
- `mod publish`: Handles publishing a repository for an organization.

In other words you can dump all the reviewed Hand On Skill assignments and a template README.md file and build the module dirs create repos sort files to the matching dir and create remotes and push that github. You would still need manualy change the each repo to a template repo and then connect it to the GitHub classroom. 