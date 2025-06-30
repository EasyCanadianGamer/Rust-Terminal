// src/commands.rs
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn execute_command(input: &str) {
    match input {
        "hello" => hello(),
        "date" => date(),
        "time" => time(),
        "clear" => clear(),
        "pwd" => pwd(),
        "ls" => ls(),
        "whoami" => whoami(),
        "help" => help(),
        _ if input.starts_with("echo ") => echo(&input[5..]),
        _ if input.starts_with("cat ") => cat(&input[4..]),
        _ if input.starts_with("touch ") => touch(&input[6..]),
        _ if input.starts_with("rm ") => rm(&input[3..]),
        _ if input.starts_with("cd ") => cd(&input[3..]),
        _ if input.starts_with("write ") => write(&input[6..]),
        _ => println!("Unknown command: {}", input),
    }
}

fn hello() {
    println!("Hello, user!");
}

fn date() {
    println!("Today's date is {}", chrono::Local::now().format("%Y-%m-%d"));
}

fn time() {
    println!("The current time is {}", chrono::Local::now().format("%H:%M:%S"));
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");  // Clear the terminal screen
    io::stdout().flush().unwrap();
}

fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error retrieving current directory: {}", e),
    }
}

fn ls() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => println!("Error listing files: {}", e),
    }
}

fn whoami() {
    match env::var("USER").or_else(|_| env::var("USERNAME")) {
        Ok(user) => println!("Current user: {}", user),
        Err(_) => println!("Could not retrieve username"),
    }
}

fn help() {
    println!("Available commands:");
    println!("  hello      - Greets the user.");
    println!("  date       - Shows the current date.");
    println!("  time       - Shows the current time.");
    println!("  clear      - Clears the screen.");
    println!("  pwd        - Prints the current working directory.");
    println!("  ls         - Lists files in the current directory.");
    println!("  whoami     - Displays the current username.");
    println!("  echo [msg] - Displays the message entered.");
    println!("  cat [file] - Displays the contents of a file.");
    println!("  touch [file] - Creates an empty file.");
    println!("  rm [file]  - Deletes the specified file.");
    println!("  write [file]  - Write in the specified file if it exists, if not create a new file.");
    println!("  cd [dir]   - Changes the current directory.");
    println!("  exit       - Exits the terminal.");
}

fn echo(message: &str) {
    println!("{}", message);
}

fn cat(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn touch(filename: &str) {
    match fs::File::create(filename) {
        Ok(_) => println!("Created file: {}", filename),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn rm(filename: &str) {
    match fs::remove_file(filename) {
        Ok(_) => println!("Deleted file: {}", filename),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

// New cd function
fn cd(path: &str) {
    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => println!("Error changing directory: {}", e),
    }
    
}
fn write(args: &str) {
    // Split args by the first space to separate filename and content
    let mut parts = args.splitn(2, ' ');
    let filename = parts.next().unwrap_or("");
    let content = parts.next().unwrap_or("");

    // Ensure both filename and content are provided
    if filename.is_empty() || content.is_empty() {
        println!("Usage: write <filename> \"<text>\"");
        return;
    }

    // Open the file in append mode (or create it if it doesn't exist)
    match OpenOptions::new().create(true).append(true).open(filename) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", content) {
                println!("Error writing to file: {}", e);
            } else {
                println!("Text written to {}", filename);
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
}
