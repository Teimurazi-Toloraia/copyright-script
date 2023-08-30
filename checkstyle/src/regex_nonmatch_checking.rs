use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_file_content(file_path: &Path) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Some(content),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::InvalidData {
                eprintln!(
                    "Warning: File '{}' contains invalid UTF-8 data, skipping...",
                    file_path.display()
                );
                None
            } else {
                eprintln!("Failed to read file '{}': {}", file_path.display(), err);
                None
            }
        }
    }
}

pub fn check_matching(file: &str, regex: &str) -> bool {
    let re = Regex::new(&format!("(?m){}", regex)).unwrap();
    re.is_match(file)
}

pub fn nonmatching_files_from_list(
    regex_file_path: &Path,
    file_paths: Vec<PathBuf>,
) -> Vec<PathBuf> {
    let regex = match read_file_content(regex_file_path) {
        Some(regex_content) => regex_content,
        None => return Vec::new(),
    };

    file_paths
        .into_iter()
        .filter(|target_file_name| {
            if let Some(content) = read_file_content(target_file_name.as_ref()) {
                !check_matching(&content, &regex)
            } else {
                false
            }
        })
        .collect()
}
