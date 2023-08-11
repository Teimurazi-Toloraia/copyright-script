use regex::Regex;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

fn main() -> Result<(), std::io::Error> {
    let regex_file_path = PathBuf::from("checkstyle-file-agpl-header.txt".to_string());
    let target_path = PathBuf::from(".".to_string());
    let (matching_files, nonmatching_files) =
        separate_regex_matching_files(&regex_file_path, &target_path);
    for file in &matching_files {
        println!("{file} is matching regex in {}", regex_file_path.display());
    }
    for file in &nonmatching_files {
        println!(
            "{file} is NOT matching regex in {}",
            regex_file_path.display()
        );
    }
    if !nonmatching_files.is_empty() {
        process::exit(1);
    }
    Ok(())
}

fn read_file_content(file_path: &Path) -> String {
    fs::read_to_string(file_path).expect("Failed to read file")
}

fn check_matching(file: &str, regex: &str) -> Option<bool> {
    let file_lines: Vec<&str> = file.lines().collect();
    let regex_lines: Vec<&str> = regex.lines().collect();
    let mut file_id = 0;
    let mut regex_id = 0;
    while file_id < file_lines.len() && regex_id < regex_lines.len() {
        let regex_line = regex_lines[regex_id];
        let file_line = file_lines[file_id];
        let re = Regex::new(regex_line).ok()?;
        if re.is_match(file_line) {
            file_id += 1;
            regex_id += 1;
        } else {
            file_id += 1;
        }
    }
    Some(regex_id == regex_lines.len())
}

fn list_files_in_folder(folder_path: &Path) -> Result<Vec<String>, std::io::Error> {
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

fn all_files(folder_path: &Path) -> Vec<String> {
    list_files_in_folder(folder_path).unwrap_or_default()
}

fn separate_regex_matching_files(
    regex_file_path: &Path,
    target_path: &Path,
) -> (Vec<String>, Vec<String>) {
    let target_file_paths = all_files(target_path);
    let regex = read_file_content(regex_file_path);

    let (matching_files, nonmatching_files): (Vec<String>, Vec<String>) =
        target_file_paths.into_iter().partition(|target_file_name| {
            let target_file = read_file_content(&target_file_name.as_ref());
            let is_match_option = check_matching(&target_file, &regex);
            is_match_option.unwrap_or(false)
        });

    (matching_files, nonmatching_files)
}

#[cfg(test)]
mod tests {
    use crate::check_matching;
    use crate::read_file_content;

    fn is_option_true(option_value: Option<bool>) -> bool {
        option_value.unwrap_or(false)
    }
    #[test]
    fn test_checker() {
        let regex = read_file_content("checkstyle-file-agpl-header.txt".as_ref());
        let file = read_file_content("Syncer.kt".as_ref());
        let result = check_matching(&file, &regex);
        assert!(is_option_true(result));
    }
}
