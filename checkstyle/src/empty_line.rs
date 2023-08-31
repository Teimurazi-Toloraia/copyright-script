use std::path::{Path, PathBuf};
use crate::regex_nonmatch::read_file_content;

fn ends_with_empty_line(file_path: &Path) -> bool {
    let file_content: String = read_file_content(file_path).unwrap();
    let lines: Vec<&str> = file_content.lines().collect();

    if let Some(last_line) = lines.last() {
        last_line.trim().is_empty()
    } else {
        false
    }
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
    #[test]
    fn empty_line() {
    }
}
