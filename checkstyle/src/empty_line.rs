use std::path::{Path, PathBuf};
use crate::regex_nonmatch::read_file_content;

fn ends_with_empty_line(file_path: &Path) -> bool {
    let file_content: String = read_file_content(file_path).unwrap();
    let lines: Vec<&str> = file_content.lines().collect();

    lines.last().is_some_and(|l| l.trim().is_empty())
}

pub fn no_empty_line(file_paths: Vec<PathBuf>) -> Vec<PathBuf> {
    file_paths
        .into_iter()
        .filter(|file_path| {
            !ends_with_empty_line(file_path)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::PathBuf;
    use crate::{glob_patterns::all_files, empty_line::no_empty_line};

    #[test]
    fn empty_line() {
        let target_path = PathBuf::from("test_folder/files");

        let file_paths = all_files(&target_path);

        let result = no_empty_line(file_paths);

        assert!(result.contains(&PathBuf::from("test_folder/files/bad.txt")));
        assert!(!result.contains(&PathBuf::from("test_folder/files/good.txt")));
    }
}
