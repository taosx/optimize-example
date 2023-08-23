#![feature(path_file_prefix)]
use anyhow::{anyhow, Result};
use std::{env, path::PathBuf};

mod optimize;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Check if at least one argument is provided (the first argument is the program name)

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [output_file]", args[0]);
        std::process::exit(1);
    }

    let input_file_path = PathBuf::from(&args[1]);
    let output_file_path = match args.get(2) {
        Some(path) => PathBuf::from(path),
        None => {
            let mut output_path = input_file_path.clone();

            let file_prefix = output_path
                .file_prefix()
                .ok_or(anyhow!("The path to <input_file> seems to be a directory."))?;

            if let Some(extension) = output_path.extension() {
                output_path.set_file_name(format!(
                    "{}.out.{}",
                    file_prefix.to_string_lossy(),
                    extension.to_string_lossy()
                ));
            } else {
                output_path.set_file_name(format!("{}.out", file_prefix.to_string_lossy()));
            }
            output_path
        }
    };

    optimize::optimize(&input_file_path, &output_file_path)
}
