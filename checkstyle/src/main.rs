use clap::Parser;
use std::path::PathBuf;
mod glob_patterns;
mod regex_nonmatch;
use glob_patterns::matching_files;
use regex_nonmatch::nonmatching_files;
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

    let file_paths = matching_files(&target_path, args.include, args.exclude);

    let nonmatching_files = nonmatching_files(&regex_file_path, file_paths);

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
