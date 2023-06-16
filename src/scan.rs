/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///

use crate::CliArgs;

use std::{sync::mpsc, time::Duration};
use colored::Colorize;
use rayon::ThreadPoolBuilder;
use reqwest::{StatusCode, blocking::Client};

pub fn spawn_threads(cli_args: CliArgs, directories: Vec<String>) {
    // Create a thread pool with the specified number of threads
    let pool = ThreadPoolBuilder::new()
        .num_threads(cli_args.threads as usize)
        .build()
        .unwrap_or_else(|err| panic!("Failed to create thread pool: {}", err));

    let (tx, rx) = mpsc::channel();

    let client = Client::new();

    for directory in directories {
        let url = format!("{}{}", cli_args.url, directory);

        let tx_clone = tx.clone();

        let client_clone = client.clone();

        pool.spawn(move || {
            // Sleep for the specified amount of time
            if cli_args.timeout > 0 {
                std::thread::sleep(Duration::from_secs(cli_args.timeout));
            }

            let resp = {
                let resp = client_clone.get(&url).send();

                match resp {
                    Ok(resp) => resp,
                    Err(err) => {
                        // Handle or report the error as needed
                        eprintln!("Error occurred for {}: {}", url.red(), err);
                        return; // Exit the thread early
                    }
                }

            };
            
            // Check the status code
            if resp.status() != StatusCode::NOT_FOUND {
                // Send the result to the main thread
                tx_clone.send((format!("{}", resp.status()), url)).unwrap();
            }
        
        });
    }

    drop(tx); // Drop the sender to signal the end of data transmission

    // Loop over the receiver, print results
    for (status, url) in rx {
        println!("{}: {}", status.green().bold(), url.green().bold());
    }
}