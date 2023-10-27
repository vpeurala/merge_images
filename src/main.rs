use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: <output_directory> <input_directory1> <input_directory2> [...<input_directoryN>]");
        std::process::exit(1);
    }

    let output_dir = PathBuf::from(&args[1]);
    let input_dirs: Vec<PathBuf> = args[2..].iter().map(|s| PathBuf::from(s)).collect();

    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Collect all image files from input directories
    let image_files: Vec<PathBuf> = input_dirs.iter()
        .flat_map(|dir| {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path())
                .filter(|path| {
                    path.is_file() && is_image_file(&path)
                })
        })
        .collect();

    // Create progress bar
    let progress_bar = ProgressBar::new(image_files.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
        .progress_chars("##-"));

    // Copy files to output directory
    for (i, file) in image_files.iter().enumerate() {
        let new_file_name = format!("{:06}.png", i + 1);
        let new_file_path = output_dir.join(new_file_name);
        fs::copy(file, &new_file_path)?;
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Images have been sorted and copied.");

    Ok(())
}

fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return ["png", "jpg", "jpeg"].contains(&&*ext);
    }
    false
}

