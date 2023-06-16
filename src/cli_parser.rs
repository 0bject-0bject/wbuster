use std::path::PathBuf;

use crate::CliArgs;
use clap::{Command, Arg};


pub fn cli() -> CliArgs {
    let matches = Command::new("wbuster")
        .version("0.1.0")
        .author("Whaledev <whaledev.contact@gmail.com>")
        .about("A simple directory brute force tool for websites")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("path")
            .required(true)
            .help("Wordlist (CSV, JSON)"))
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .value_name("url")
            .required(true)
            .help("URL to search."))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .value_name("threads")
            .required(false)
            .help("Number of threads to use."))
        .arg(Arg::new("timeout")
            .long("timeout")
            .value_name("timeout")
            .required(false)
            .help("Timeout for each request in seconds."))
        .get_matches();


    let path = matches
        .get_one::<String>("path")
        .unwrap_or_else(|| panic!("Invalid path"))
        .parse::<PathBuf>()
        .unwrap();

    let mut url = matches
        .get_one::<String>("url")
        .unwrap_or_else(|| panic!("Invalid url"))
        .to_string();

    let threads = matches
        .get_one::<String>("threads")
        .unwrap_or(&String::from("2"))
        .parse::<u32>()
        .unwrap_or(2);

    let timeout = matches
        .get_one::<String>("timeout")
        .unwrap_or(&String::from("0"))
        .parse::<u64>()
        .unwrap_or(0);

    // Check if the url ends with a slash
    if !url.ends_with("/") {
        url.push_str("/");
    }

    let cli_args = CliArgs {
        path,
        url,
        threads,
        timeout
    };

    return cli_args;
}