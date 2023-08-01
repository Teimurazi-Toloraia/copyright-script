fn get_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read schema file")
}

fn check_matching(file: &str, regex: &str) -> Boolean {
    // TODO
}

fn all_files(target_path: &str) -> Vec<String> {
    // TODO
}

fn solve(regex_file_path: &str, target_path: &str) {
    target_files = all_files(target_path);
}