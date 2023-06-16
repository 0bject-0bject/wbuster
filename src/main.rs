/// 
/// Copyright 2023, Whaledev
/// Licensed under MIT
///
/// 🦥 THX FOR LOOKING! <3 🦥
/// 
/// Warnings:
/// DO NOT USE THIS TOOL FOR ILLEGAL PURPOSES, 
/// THIS TOOL IS FOR EDUCATIONAL PURPOSES ONLY
/// 
/// This tool is not finished yet, it is still in development! 🟥
/// 
/// This tool can overload servers, so be careful! 🟨
/// 
/// 
/// Simple test express server:
/// https://github.com/0bject-0bject/Express-Test-Server

use std::{path::PathBuf};

mod read_file;
mod graphics;
mod scan;
mod cli_parser;

pub struct CliArgs {
    path: PathBuf,
    url: String,
    threads: u32,
    timeout: u64
}

fn main() {
    let cli_args = cli_parser::cli();

    // Read the file, and process for directories
    let directories = read_file::read(&cli_args.path);

    // Clear the screen
    clear_screen();

    // Print the graphic
    graphics::wscan_graphic(&cli_args.url);

    // Start the scan
    scan::spawn_threads(cli_args, directories);

    // Exit the program
    std::process::exit(0);
}

pub fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}