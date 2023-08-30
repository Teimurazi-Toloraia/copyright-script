use clap::Parser;
use std::path::PathBuf;
mod files_matching_glob_patterns;
mod regex_nonmatch_checking;
use files_matching_glob_patterns::files_matching_patterns;
use regex_nonmatch_checking::nonmatching_files_from_list;
use std::io::{Error, ErrorKind};

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

    let file_paths = files_matching_patterns(&target_path, args.include, args.exclude);

    let nonmatching_files = nonmatching_files_from_list(&regex_file_path, file_paths);

    for file in &nonmatching_files {
        println!(
            "File {} does not match regex: {}",
            file.display(),
            regex_file_path.display()
        );
    }

    if !nonmatching_files.is_empty() {
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{files_matching_patterns, nonmatching_files_from_list, PathBuf};
    use crate::regex_nonmatch_checking::{check_matching, read_file_content};
    use std::path::Path;

    fn regex_nonmatching_files(
        regex_file_path: &Path,
        target_path: &Path,
        include_patterns: Option<Vec<String>>,
        exclude_patterns: Option<Vec<String>>,
    ) -> Vec<PathBuf> {
        let file_paths = files_matching_patterns(target_path, include_patterns, exclude_patterns);
        nonmatching_files_from_list(regex_file_path, file_paths)
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
