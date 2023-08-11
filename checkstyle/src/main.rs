use regex::Regex;
use std::fs::{self};

fn get_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

fn split_into_lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn check_matching_regex_line(file: &str, re: Regex) -> Option<bool> {
    let file_lines = split_into_lines(file);
    for file_line in file_lines {
        if re.is_match(file_line) {
            return Some(true);
        }
    }
    Some(false)
}

fn check_matching(file: &str, regex: &str) -> Option<bool> {
    let lines: Vec<&str> = split_into_lines(regex);
    for regex_line in lines {
        let re = Regex::new(regex_line).ok()?;
        if check_matching_regex_line(file, re) == Some(false) {
            return Some(false);
        }
    }
    Some(true)
}

fn list_files_in_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    file_paths.push(file_name_str.to_string());
                }
            }
        }
    }

    Ok(file_paths)
}

fn all_files(folder_path: &str) -> Vec<String> {
    match list_files_in_folder(folder_path) {
        Ok(file_paths) => {
            return file_paths;
        }
        _ => {
            return Vec::new();
        }
    }
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

use std::process;

fn main() {
    let regex_file_path: String = String::from("checkstyle-file-agpl-header.txt");
    let target_path: String = ".".to_string();
    let (matching_files, nonmatching_files) = solve(&regex_file_path, &target_path);
    for file in &matching_files {
        println!("{file} is a matching file");
    }
    for file in &nonmatching_files {
        println!("{file} is a nonmatching file");
    }
    // Check if there are non-matching files and return the appropriate exit code
    if !nonmatching_files.is_empty() {
        process::exit(1); // Exit with code 1 for non-matching files
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
