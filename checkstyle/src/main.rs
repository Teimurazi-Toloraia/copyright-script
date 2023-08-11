use regex::Regex;
use std::fs;
use std::process;

fn main() -> Result<(), std::io::Error> {
    let regex_file_path = String::from("checkstyle-file-agpl-header.txt");
    let target_path = ".".to_string();
    let (matching_files, nonmatching_files) =
        separate_regex_matching_files(&regex_file_path, &target_path);
    for file in &matching_files {
        println!("{file} is matching regex in {regex_file_path}");
    }
    for file in &nonmatching_files {
        println!("{file} is NOT matching regex in {regex_file_path}");
    }
    if !nonmatching_files.is_empty() {
        process::exit(1);
    }
    Ok(())
}

fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file")
}

fn check_matching_regex_line(file: &str, re: Regex) -> bool {
    for file_line in file.lines() {
        if re.is_match(file_line) {
            return true;
        }
    }
    false
}

fn check_matching(file: &str, regex: &str) -> Option<bool> {
    for regex_line in regex.lines() {
        let re = Regex::new(regex_line).ok()?;
        if !check_matching_regex_line(file, re) {
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
    list_files_in_folder(folder_path).unwrap_or_default()
}

fn separate_regex_matching_files(
    regex_file_path: &str,
    target_path: &str,
) -> (Vec<String>, Vec<String>) {
    let target_file_paths = all_files(target_path);
    let regex = read_file_content(regex_file_path);

    let (matching_files, nonmatching_files): (Vec<String>, Vec<String>) =
        target_file_paths.into_iter().partition(|target_file_name| {
            let target_file = read_file_content(&target_file_name);
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
        let regex = read_file_content("checkstyle-file-agpl-header.txt");
        let file = read_file_content("Syncer.kt");
        println!("regex is {regex}");
        println!("file is {file}");
        let result = check_matching(&file, &regex);
        assert!(is_option_true(result));
    }
}
