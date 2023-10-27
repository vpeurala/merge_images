use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <output_dir> <input_dir1> <input_dir2> [...]", args[0]);
        std::process::exit(1);
    }
    let output_dir = PathBuf::from(&args[1]);
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    let mut all_files: Vec<Vec<PathBuf>> = vec![];

    for input_dir in &args[2..] {
        let mut file_paths = vec![];
        let input_path = PathBuf::from(input_dir);
        if input_path.exists() && input_path.is_dir() {
            for entry in fs::read_dir(input_path)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    file_paths.push(entry.path());
                }
            }
        }
        file_paths.sort();
        all_files.push(file_paths);
    }

    let max_len = all_files.iter().map(|v| v.len()).max().unwrap_or(0);

    let pb = ProgressBar::new((max_len * all_files.len()) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"),
    );

    let mut counter = 1;
    for i in 0..max_len {
        for files in &all_files {
            if i < files.len() {
                let new_name = format!("{:06}.png", counter);
                let new_path = output_dir.join(new_name);
                fs::copy(&files[i], &new_path)?;
                pb.inc(1);
                counter += 1;
            }
        }
    }

    pb.finish_with_message("Done!");
    Ok(())
}

