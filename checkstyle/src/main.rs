use regex::Regex;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

fn main() -> Result<(), std::io::Error> {
    let regex_file_path = PathBuf::from("checkstyle-file-agpl-header.txt".to_string());
    let target_path = PathBuf::from(".".to_string());
    let nonmatching_files = separate_regex_matching_files(&regex_file_path, &target_path);
    for file in &nonmatching_files {
        println!(
            "File {} does not match regex: {}",
            file.display(),
            regex_file_path.display()
        );
    }
    if !nonmatching_files.is_empty() {
        process::exit(1);
    }
    Ok(())
}

fn read_file_content(file_path: &Path) -> Option<String> {
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

fn check_direct_matching(file_lines: Vec<&str>, regex_lines: Vec<&str>) -> Option<bool> {
    if file_lines.len() < regex_lines.len() {
        return Some(false);
    }
    let together = file_lines.iter().zip(regex_lines.iter());
    for (file_line, regex_line) in together {
        let re = Regex::new(regex_line).ok()?;
        if !re.is_match(file_line) {
            return Some(false);
        }
    }
    Some(true)
}

fn check_matching(file: &str, regex: &str) -> bool {
    let file_lines: Vec<&str> = file.lines().collect();
    let regex_lines: Vec<&str> = regex.lines().collect();
    for i in 0..file_lines.len() {
        let suffix = &file_lines[i..];
        if check_direct_matching(suffix.to_vec(), regex_lines.to_vec()).unwrap_or(false) {
            return true;
        }
    }
    return false;
}

fn list_files_in_folder(folder_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            file_paths.push(path.clone());
        } else if path.is_dir() {
            if let Ok(sub_paths) = list_files_in_folder(&path) {
                file_paths.extend(sub_paths);
            }
        }
    }

    Ok(file_paths)
}

fn all_files(folder_path: &Path) -> Vec<PathBuf> {
    list_files_in_folder(folder_path).unwrap_or_default()
}

fn separate_regex_matching_files(regex_file_path: &Path, target_path: &Path) -> Vec<PathBuf> {
    let target_file_paths = all_files(target_path);
    let regex = match read_file_content(regex_file_path) {
        Some(regex_content) => regex_content,
        None => return Vec::new(), // Early return if regex file couldn't be read
    };
    let (_matching_files, nonmatching_files) =
        target_file_paths.into_iter().partition(|target_file_name| {
            let target_file = match read_file_content(&target_file_name.as_ref()) {
                Some(content) => content,
                None => return false,
            };
            check_matching(&target_file, &regex)
        });

    nonmatching_files
}

#[cfg(test)]
mod tests {
    use crate::check_matching;
    use crate::read_file_content;

    #[test]
    fn test_checker() {
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
}
