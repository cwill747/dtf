use std::env;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::process;
use tempfile::{Builder};
use reqwest::blocking::get;
use reqwest::Url;
use anyhow::{anyhow, Result};

fn download_file(url: &str) -> Result<PathBuf> {
    let parsed_url = Url::parse(url)?;
    let filename = parsed_url
        .path_segments()
        .and_then(|segments| segments.last())
        .ok_or(anyhow!("Error: Failed to extract the filename from the URL."))?;

    let temp_dir = Builder::new()
        .keep(true)
        .tempdir()?;
    let file_path = temp_dir.path().join(filename);

    let response = get(url)?;
    if !response.status().is_success() {
        return Err(anyhow!("Error: Failed to download file. HTTP Status: {}", response.status()));
    }

    let mut dest = File::create(&file_path)?;
    let mut content = response;
    copy(&mut content, &mut dest)?;

    Ok(file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: download_file <URL>");
        process::exit(1);
    }

    let url = &args[1];

    match download_file(url) {
        Ok(file_path) => {
            println!("{}", file_path.display());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}