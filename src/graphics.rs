/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

use colored::Colorize;

// Literally just prints the banner😭
pub fn wscan_graphic(url: &String) {

    print!("{}", " _       __ ____   __  __ _____ ______ ______ ____ \r\n".bright_blue().bold());
    print!("{}", "| |     / // __ ) / / / // ___//_  __// ____// __ \\\r\n".bright_blue().bold());
    print!("{}", "| | /| / // __  |/ / / / \\__ \\  / /  / __/  / /_/ /\r\n".bright_blue().bold());
    print!("{}", "| |/ |/ // /_/ // /_/ / ___/ / / /  / /___ / _, _/ \r\n".bright_blue().bold());
    print!("{}", "|__/|__//_____/ \\____/ /____/ /_/  /_____//_/ |_|  \r\n".bright_blue().bold());
    print!("\r\n");

    print!("{} {}\r\n", "Starting on".cyan().bold(), url.cyan().bold().underline());
    print!("\r\n");
}
