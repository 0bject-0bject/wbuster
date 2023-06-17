/// 
/// Copyright 2023, [object Object]
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

mod scan;
mod cli_parser;
mod cli_args;

use std::{process};
use colored::control;
use crate::cli_args::CliArgs;

fn main() {
    // Enable virtual terminal processing on windows
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    // Parse the cli arguments
    let cli_args = match cli_parser::cli() {
        Ok(cli_args) => cli_args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Clear the screen
    clear_screen();

    // Start the scan
    scan::spawn_threads(cli_args.0, cli_args.1);

    // Exit the program
    process::exit(0);
}

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}