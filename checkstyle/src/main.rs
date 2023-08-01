use std::fs::{self};
use text_io::read;
use regex::Regex;


fn get_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read schema file")
}

fn check_matching(file: &str, regex: &str) -> bool {
    let rx = Regex::new(regex).unwrap();
    true
    // TODO
}

fn all_files(target_path: &str) -> Vec<&str> {
    vec![]
    // TODO
}

fn solve(regex_file_path: &str, target_path: &str) -> (Vec<String>, Vec<String>) {
    let target_file_paths = all_files(target_path);
    let regex = get_file(regex_file_path);
    let mut matching_files: Vec<String> = Vec::new();
    let mut nonmatching_files: Vec<String> = Vec::new();

    for target_file_name in target_file_paths {
        let target_file = get_file(&target_file_name);
        if check_matching(&target_file, &regex) {
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
