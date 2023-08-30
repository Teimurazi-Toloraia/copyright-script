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

pub fn nonmatching_files(
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

#[cfg(test)]
mod tests {
    use super::PathBuf;    
    use crate::glob_patterns::matching_files;
    use crate::regex_nonmatch::{check_matching, read_file_content, nonmatching_files};
    use std::path::Path;

    fn regex_nonmatching_files(
        regex_file_path: &Path,
        target_path: &Path,
        include_patterns: Option<Vec<String>>,
        exclude_patterns: Option<Vec<String>>,
    ) -> Vec<PathBuf> {
        let file_paths = matching_files(target_path, include_patterns, exclude_patterns);
        nonmatching_files(regex_file_path, file_paths)
    }

    #[test]
    fn matching() {
        let regex = match read_file_content("checkstyle-file-agpl-header.txt".as_ref()) {
            Some(content) => content,
            None => {
                println!("Failed to read regex file");
                return;
            }
        };
        let file = match read_file_content("Syncer.kt".as_ref()) {
            Some(content) => content,
            None => {
                println!("Failed to read file");
                return;
            }
        };
        assert!(check_matching(&file, &regex));
    }

    #[test]
    fn separator() {
        let include_patterns = vec!["*".to_string(), "*/*".to_string()];
        let exclude_patterns = vec!["*/*.txt".to_string()];
        let regex_file_path = PathBuf::from("checkstyle-file-agpl-header.txt".to_string());
        let target_path = PathBuf::from(".".to_string());
        let result = regex_nonmatching_files(
            &regex_file_path,
            &target_path,
            Some(include_patterns),
            Some(exclude_patterns),
        );
        assert!(!result.contains(&PathBuf::from("Syncer.kt")));
    }

    #[test]
    fn inclusion_and_exclusion() {
        let include_patterns = vec!["**/*.kt".to_string()];
        let exclude_patterns = vec!["dir2/*".to_string()];
        let regex_file_path = PathBuf::from("checkstyle-file-agpl-header.txt");
        let target_path = PathBuf::from("test_folder");

        let result = regex_nonmatching_files(
            &regex_file_path,
            &target_path,
            Some(include_patterns),
            Some(exclude_patterns),
        );

        assert!(!result.contains(&PathBuf::from("test_folder/dir1/Syncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/Syncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir3/Syncer.txt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/modifiedSyncer.kt")));
    }
}
