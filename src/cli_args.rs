/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

use std::path::PathBuf;

pub struct CliArgs {
    pub path: PathBuf,
    pub url: String,
    pub threads: u32,
    pub timeout: u64,
}