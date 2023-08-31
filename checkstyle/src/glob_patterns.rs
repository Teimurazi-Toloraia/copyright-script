use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};

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

pub fn matching_files(
    target_path: &Path,
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
) -> Vec<PathBuf> {
    let target_file_paths = all_files(target_path);
    let include_globs = include_patterns.unwrap_or(vec!["**/*".to_string()]);
    let exclude_globs = exclude_patterns.unwrap_or(Vec::new());
    target_file_paths
        .into_iter()
        .filter(|target_file_name| {
            let file_name = target_file_name.file_name().unwrap().to_str().unwrap();

            let include_match = include_globs.iter().any(|include_glob| {
                let pattern = Pattern::new(include_glob).unwrap();
                pattern.matches(file_name)
            });

            let exclude_match = exclude_globs.iter().any(|exclude_glob| {
                let pattern = Pattern::new(exclude_glob).unwrap();
                pattern.matches(file_name)
            });

            include_match && !exclude_match
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::PathBuf;    
    use crate::glob_patterns::matching_files;

    #[test]
    fn inclusion_and_exclusion() {
        let include_patterns = vec!["**/*.kt".to_string(), "**/*.txt".to_string()];
        let exclude_patterns = vec!["dir2/*".to_string()];
        let target_path = PathBuf::from("test_folder");

        let result = matching_files(
            &target_path,
            Some(include_patterns),
            Some(exclude_patterns),
        );

        assert!(result.contains(&PathBuf::from("test_folder/dir1/Syncer.kt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/Syncer.txt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/modifiedSyncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/Syncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/modifiedSyncer.kt")));
    }
}
