use crate::regex_nonmatch::read_file_content;
use std::path::{Path, PathBuf};

fn ends_with_empty_line(file_path: &Path) -> bool {
    let file_content: String = read_file_content(file_path).unwrap();

    if file_content.is_empty() {
        return false;
    }

    let last_char = file_content.chars().last().unwrap();

    // Check if the last character is a newline character ('\n')
    last_char == '\n'
}

pub fn no_empty_line(file_paths: Vec<PathBuf>) -> Vec<PathBuf> {
    file_paths
        .into_iter()
        .filter(|file_path| !ends_with_empty_line(file_path))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::PathBuf;
    use crate::{empty_line::no_empty_line, glob_patterns::all_files};

    #[test]
    fn empty_line() {
        let target_path = PathBuf::from("test_folder/files");

        let file_paths = all_files(&target_path);

        let result = no_empty_line(file_paths);

        assert!(result.contains(&PathBuf::from("test_folder/files/bad.txt")));
        assert!(!result.contains(&PathBuf::from("test_folder/files/good.txt")));
    }
}
