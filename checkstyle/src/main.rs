use regex::Regex;
use std::fs::{self};
use text_io::read;
// use std::path::Path;
// use std::path::PathBuf;
// use std::fs::File;
// use std::io::prelude::*;

fn get_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

fn check_matching(file: &str, regex: &str) -> Option<bool> {
    let re = Regex::new(regex).ok()?; // Use .ok() to convert Result to Option
    Some(re.is_match(file))
}

fn all_files(_dir: &str) -> Vec<&str> {
    return vec!["Syncer.kt"];
    // let mut files: Vec<&str> = Vec::new();
    // for file in dir {
    //     if let Ok(file) = file {
    //         if file.file_type().unwrap().is_file() {
    //             files.push(file.path());
    //         } else if file.file_type().unwrap().is_dir() {
    //             let rdir = file.path().read_dir().unwrap();
    //         }
    //     }
    // }
    // files
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
    print!("Type regex file path"); // checkstyle-file-agpl-header.txt
    let regex_file_path: String = String::from("checkstyle-file-agpl-header.txt");
    // let regex_file_path: String = read!();
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

#[cfg(test)]
mod tests {
    use crate::check_matching;
    use crate::get_file;

    fn is_option_true(option_value: Option<bool>) -> bool {
        match option_value {
            Some(true) => true,
            _ => false,
        }
    }
    #[test]
    fn test_checker() {
        let regex = get_file("checkstyle-file-agpl-header.txt");
        let file = get_file("Syncer.kt");
        println!("regex is {regex}");
        println!("file is {file}");
        let result = check_matching(&file, &regex);
        assert!(is_option_true(result));
    }
}
