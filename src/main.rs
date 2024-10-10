use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::process;
use tempfile::{Builder};
use reqwest::blocking::get;
use error_chain::{bail, error_chain};
use clap::Parser;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn download_file(url: reqwest::Url) -> Result<PathBuf> {
    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .ok_or("Error: Failed to extract the filename from the URL.")?;

    let temp_dir = Builder::new()
        .keep(true)
        .tempdir()?;
    let file_path = temp_dir.path().join(filename);

    let response = get(url)?;
    if !response.status().is_success() {
        bail!("Error: Failed to download file. HTTP Status: {}", response.status());
    }

    let mut dest = File::create(&file_path)?;
    let mut content = response;
    copy(&mut content, &mut dest)?;

    Ok(file_path)
}

#[derive(Parser)]
struct Cli {
    /// the URL to download
    path: reqwest::Url,
}

fn main() {
    let args = Cli::parse();


    match download_file(args.path) {
        Ok(file_path) => {
            println!("{}", file_path.display());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}