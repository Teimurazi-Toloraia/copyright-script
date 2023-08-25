use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    target_path: String,

    #[arg(short, long)]
    regex_file_path: String,

    #[arg(short, long)]
    include: Option<Vec<String>>,
    #[arg(short, long)]
    exclude: Option<Vec<String>>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let regex_file_path = PathBuf::from(&args.regex_file_path);
    let target_path = PathBuf::from(&args.target_path);

    let include_patterns = args.include;
    let exclude_patterns = args.exclude;

    let nonmatching_files = separate_regex_matching_files(
        &regex_file_path,
        &target_path,
        include_patterns,
        exclude_patterns,
    );

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

fn check_matching(file: &str, regex: &str) -> bool {
    let re = Regex::new(&format!("(?m){}", regex)).unwrap();
    re.is_match(file)
}

fn list_files_in_folder(folder_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_symlink() {
            continue;
        }

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

fn separate_regex_matching_files(
    regex_file_path: &Path,
    target_path: &Path,
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
) -> Vec<PathBuf> {
    let target_file_paths = all_files(target_path);
    let regex = match read_file_content(regex_file_path) {
        Some(regex_content) => regex_content,
        None => return Vec::new(),
    };

    let include_globs = include_patterns.unwrap_or(vec!["**/*".to_string()]);
    let exclude_globs = exclude_patterns.unwrap_or(Vec::new());

    target_file_paths
        .into_iter()
        .filter(|target_file_name| {
            let file_name = target_file_name.file_name().unwrap().to_str().unwrap();

            let mut include_match = false;
            for include_glob in &include_globs {
                let pattern = glob::Pattern::new(include_glob).unwrap();
                if pattern.matches(file_name) {
                    include_match = true;
                    break;
                }
            }

            let mut exclude_match = false;
            for exclude_glob in &exclude_globs {
                let pattern = glob::Pattern::new(exclude_glob).unwrap();
                if pattern.matches(file_name) {
                    exclude_match = true;
                    break;
                }
            }

            if let Some(content) = read_file_content(target_file_name.as_ref()) {
                include_match && !exclude_match && !check_matching(&content, &regex)
            } else {
                false
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_matching;
    use crate::read_file_content;

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
        let exclude_patterns = vec![
            "*/*.png".to_string(),
            "*/*.gif".to_string(),
            "*/*.dot".to_string(),
        ];
        let regex_file_path = PathBuf::from("checkstyle-file-agpl-header.txt".to_string());
        let target_path = PathBuf::from(".".to_string());
        let result = separate_regex_matching_files(
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

        let result = separate_regex_matching_files(
            &regex_file_path,
            &target_path,
            Some(include_patterns),
            Some(exclude_patterns),
        );

        assert!(!result.contains(&PathBuf::from("test_folder/dir1/Syncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/Syncer.kt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/modifiedSyncer.kt")));
    }
}
