use glob::Pattern;
use std::path::{Path, PathBuf};
use crate::utils::all_files;

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
        .filter(|target_file_path| {
            let include_match = include_globs.iter().any(|include_glob| {
                let pattern = Pattern::new(include_glob).unwrap();
                pattern.matches_path(&target_file_path)
            });

            let exclude_match = exclude_globs.iter().any(|exclude_glob| {
                let pattern = Pattern::new(exclude_glob).unwrap();
                pattern.matches_path(&target_file_path)
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
        let exclude_patterns = vec!["test_folder/dir2/*".to_string()];
        let target_path = PathBuf::from("test_folder");

        let result = matching_files(&target_path, Some(include_patterns), Some(exclude_patterns));

        assert!(result.contains(&PathBuf::from("test_folder/dir1/Syncer.kt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/Syncer.txt")));
        assert!(result.contains(&PathBuf::from("test_folder/dir3/modifiedSyncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/Syncer.kt")));
        assert!(!result.contains(&PathBuf::from("test_folder/dir2/modifiedSyncer.kt")));
    }
}
