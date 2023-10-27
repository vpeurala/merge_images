use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use indicatif::ProgressBar;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <output_dir> <input_dir_1> <input_dir_2> ...", args[0]);
        std::process::exit(1);
    }

    let output_dir = &args[1];
    let input_dirs: Vec<&String> = args[2..].iter().collect();

    let mut files = Vec::new();
    let mut total_files = 0;
    for dir in input_dirs.iter() {
        let entries = fs::read_dir(dir).unwrap_or_else(|_| panic!("Failed to read directory: {}", dir));
        let mut dir_files: Vec<String> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension() == Some(OsStr::new("png")))
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        dir_files.sort();
        total_files += dir_files.len();
        files.push(dir_files);
    }

    fs::create_dir_all(output_dir).unwrap_or_else(|_| panic!("Failed to create output directory: {}", output_dir));

    let progress_bar = ProgressBar::new(total_files as u64);
    let mut count = 1;
    loop {
        let mut did_something = false;
        for dir_files in files.iter_mut() {
            if let Some(file) = dir_files.pop() {
                let new_name = format!("{}/{:06}.png", output_dir, count);
                fs::copy(&file, &new_name).unwrap_or_else(|_| panic!("Failed to copy file: {} to {}", file, new_name));
                count += 1;
                progress_bar.inc(1);
                did_something = true;
            }
        }
        if !did_something {
            break;
        }
    }
    progress_bar.finish();
}
