use std::fs;
use std::ffi::OsStr;
use clap::{App, Arg};
use indicatif::ProgressBar;

fn main() {
    let matches = App::new("Image Interleaver")
        .about("Interleaves images from multiple directories into a single directory.")
        .arg(
            Arg::with_name("OUTPUT_DIR")
                .help("Sets the output directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT_DIRS")
                .help("Sets the input directories")
                .required(true)
                .multiple(true)
                .index(2),
        )
        .arg(
            Arg::with_name("SEGMENT_LENGTH")
                .help("Sets the segment length")
                .short("s")
                .long("segment-length")
                .takes_value(true)
                .default_value("1"),
        )
        .get_matches();

    let output_dir = matches.value_of("OUTPUT_DIR").unwrap();
    let input_dirs: Vec<&str> = matches.values_of("INPUT_DIRS").unwrap().collect();
    let segment_length: usize = matches
        .value_of("SEGMENT_LENGTH")
        .unwrap()
        .parse()
        .expect("SEGMENT_LENGTH must be a positive integer");

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
            for _ in 0..segment_length {
                if let Some(file) = dir_files.pop() {
                    let new_name = format!("{}/{:06}.png", output_dir, count);
                    fs::copy(&file, &new_name).unwrap_or_else(|_| panic!("Failed to copy file: {} to {}", file, new_name));
                    count += 1;
                    progress_bar.inc(1);
                    did_something = true;
                }
            }
        }
        if !did_something {
            break;
        }
    }
    progress_bar.finish();
}
