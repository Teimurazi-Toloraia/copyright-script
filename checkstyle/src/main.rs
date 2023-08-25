use clap::Parser;
use std::path::PathBuf;
use std::process;
mod separate;
use separate::separate_regex_matching_files;

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
