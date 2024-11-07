// // src/main.rs
// mod commands;

// use rustyline::Editor;
// use rustyline::error::ReadlineError;
// use std::env;

// fn main() {
//     let mut rl = Editor::<()>::new().expect("Failed to create line editor");

//     // Load history from file if it exists
//     if rl.load_history("history.txt").is_err() {
//         println!("No previous history found.");
//     }

//     loop {
//         // Get the current directory for the prompt
//         let prompt = match env::current_dir() {
//             Ok(dir) => format!("{} > ", dir.display()),
//             Err(_) => "> ".to_string(),
//         };

//         // Display the prompt and read the input
//         match rl.readline(&prompt) {
//             Ok(line) => {
//                 let input = line.trim();

//                 // Add command to history if not empty
//                 if !input.is_empty() {
//                     rl.add_history_entry(input);
//                 }

//                 // Check for exit command
//                 if input == "exit" {
//                     println!("Goodbye!");
//                     break;
//                 }

//                 // Execute the command
//                 commands::execute_command(input);
//             }
//             Err(ReadlineError::Interrupted) => {
//                 println!("CTRL-C detected, exiting...");
//                 break;
//             }
//             Err(ReadlineError::Eof) => {
//                 println!("CTRL-D detected, exiting...");
//                 break;
//             }
//             Err(err) => {
//                 println!("Error reading line: {:?}", err);
//                 break;
//             }
//         }
//     }

//     // Save history to a file
//     rl.save_history("history.txt").expect("Failed to save history");
// }
// src/main.rs
mod commands;

use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::env;
use std::process::Command;

fn main() {
    let mut rl = Editor::<()>::new().expect("Failed to create line editor");

    // Load history from file if it exists
    if rl.load_history("history.txt").is_err() {
        println!("No previous history found.");
    }

    loop {
        // Get the current directory for the prompt
        let prompt = format_prompt();

        // Display the prompt and read the input
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();

                // Add command to history if not empty
                if !input.is_empty() {
                    rl.add_history_entry(input);
                }

                // Check for exit command
                if input == "exit" {
                    println!("Goodbye!");
                    break;
                }

                // Execute the command
                commands::execute_command(input);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C detected, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D detected, exiting...");
                break;
            }
            Err(err) => {
                println!("Error reading line: {:?}", err);
                break;
            }
        }
    }

    // Save history to a file
    rl.save_history("history.txt").expect("Failed to save history");
}

// Function to format the prompt
fn format_prompt() -> String {
    // Get the username
    let username = match env::var("USER").or_else(|_| env::var("USERNAME")) {
        Ok(user) => user,
        Err(_) => "user".to_string(),
    };

    // Get the hostname
    let hostname = match Command::new("hostname").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "hostname".to_string(),
    };

    // Get the current directory and extract only the last part (folder name)
    let current_dir = match env::current_dir() {
        Ok(dir) => dir.file_name() // Get the last component of the path
            .map(|name| name.to_string_lossy().to_string()) // Convert OsStr to String
            .unwrap_or_else(|| "unknown".to_string()), // Use "unknown" if file_name is None
        Err(_) => "unknown".to_string(), // Fallback for error
    };

    // Format the prompt like "username@hostname current_folder % "
    format!("{}@{} {} > ", username, hostname, current_dir)
}
