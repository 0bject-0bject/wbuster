/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

use std::{path::PathBuf, fs::read_to_string, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
struct Directories {
    directories: Vec<String>,
}

pub fn read(path: &PathBuf) -> Vec<std::string::String> {
    if path.extension().unwrap() == "json" {
        let data = read_to_string(path).expect("Unable to read file");

        // Must be formatted like this:
        // {
        //     "directories": [
        //         "admin",
        //         "login",
        //         "etc"
        //     ]    
        // }
        
        let formatted_data: Directories = serde_json::from_str(&data).expect("JSON was not well-formatted");

        return formatted_data.directories;
    } else if path.extension().unwrap() == "txt" {
        let data = read_to_string(path).expect("Unable to read file");

        // Filter out comments and empty lines! Works with dirbuster word lists :)
        // Recommend https://tinyurl.com/yrfauyyz
        // must be seperated by newlines!!!
        let formatted_data: Directories = Directories {
            directories: data
                .split('\n')
                .filter(|s| !s.starts_with('#'))
                .map(|s| s.replace("\r", ""))
                .collect(),
        };

        return formatted_data.directories;

    } else {
        eprintln!("File extension not supported");

        exit(1);
    }
}