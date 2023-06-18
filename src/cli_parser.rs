/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

use std::{error::Error, path::PathBuf, process};

use clap::{Command, Arg};
use serde::Deserialize;

use crate::CliArgs;

#[derive(Deserialize)]
struct Directories {
    directories: Vec<String>,
}

pub fn cli() -> Result<(CliArgs, Vec<String>), &'static str> {
    let matches = Command::new("wbuster")
        .version("0.1.4")
        .author("[object Object] <whaledev.contact@gmail.com>")
        .about("A simple directory brute force tool for websites")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("path")
                .required(true)
                .help("Wordlist (CSV, JSON)"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("url")
                .required(true)
                .help("URL to search."),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("threads")
                .required(false)
                .help("Number of threads to use."),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("timeout")
                .required(false)
                .help("Timeout for each request in seconds."),
        )
        .get_matches();

    let path = matches
        .get_one::<String>("path")
        .ok_or("Invalid path")
        .map(PathBuf::from)?;
        
    let mut url = matches
        .get_one::<String>("url")
        .ok_or("Invalid url")
        .map(String::from)?;

    if !url.ends_with('/') {
        url.push('/');
    }

    let threads = matches
        .get_one::<String>("threads")
        .unwrap_or(&"2".to_string())
        .parse::<u32>()
        .map_err(|_| "Invalid threads")?;
        
    let timeout = matches
        .get_one::<String>("timeout")
        .unwrap_or(&"0".to_string())
        .parse::<u64>()
        .map_err(|_| "Invalid timeout")?;

    let cli_args = CliArgs {
        path,
        url,
        threads,
        timeout,
    };

    let directories = match read(&cli_args.path) {
        Ok(directories) => directories,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    Ok((cli_args, directories))
}

fn read(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let extension = path.extension().ok_or("File has no extension")?;

    let data = std::fs::read_to_string(path)?;

    let directories = match extension.to_str() {
        Some("json") => {
            let formatted_data: Directories =
                serde_json::from_str(&data).map_err(|e| format!("JSON was not well-formatted: {}", e))?;
            formatted_data.directories
        }
        Some("txt") => {
            //
            let directories: Vec<String> = data
                .lines() // Split the data into lines
                .filter(|s| !s.starts_with('#')) // Ignore comments
                .map(|s| s.replace("\r", "")) // Remove carriage returns
                .collect();
            directories
        }
        _ => return Err("File extension not supported".into()),
    };

    Ok(directories)
}