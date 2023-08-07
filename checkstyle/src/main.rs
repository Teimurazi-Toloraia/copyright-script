use regex::Regex;
use std::fs::{self};
use text_io::read;
// use std::path::Path;
// use std::path::PathBuf;
// use std::fs::File;
// use std::io::prelude::*;

fn get_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read schema file")
}

fn check_matching(file: &str, regex: &str) -> Option<bool> {
    let re = Regex::new(regex).ok()?; // Use .ok() to convert Result to Option
    Some(re.is_match(file))
}

fn all_files(dir: &str) -> Vec<&str> {
    let mut files: Vec<&str> = Vec::new();
    for file in dir {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                files.push(file.path());
            } else if file.file_type().unwrap().is_dir() {
                // Extract all files, even from directories.
                // Doesn't follow symlinks.
                let rdir = file.path().read_dir().unwrap();
                files.append(&mut files_from_dir(rdir));
            }
        }
    }
    files
}

fn solve(regex_file_path: &str, target_path: &str) -> (Vec<String>, Vec<String>) {
    let target_file_paths = all_files(target_path);
    let regex = get_file(regex_file_path);
    let mut matching_files: Vec<String> = Vec::new();
    let mut nonmatching_files: Vec<String> = Vec::new();

    for target_file_name in target_file_paths {
        let target_file = get_file(&target_file_name);
        let is_match_option = check_matching(&target_file, &regex);
        let is_match: bool = match is_match_option {
            Some(inner_value) => inner_value,
            None => false,
        };
        if is_match {
            matching_files.push(target_file_name.to_string());
        } else {
            nonmatching_files.push(target_file_name.to_string());
        }
    }

    (matching_files, nonmatching_files)
}

fn main() {
    print!("Type regex file path");
    let regex_file_path: String = read!();
    print!("Type target path");
    let target_path: String = read!();
    let (matching_files, nonmatching_files) = solve(&regex_file_path, &target_path);
    for file in matching_files {
        println!("{file} is a matching file");
    }
    for file in nonmatching_files {
        println!("{file} is a nonmatching file");
    }
}
