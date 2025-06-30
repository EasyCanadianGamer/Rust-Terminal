use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn execute_command(input: &str) -> String {
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
        _ => format!("Unknown command: {}", input),
    }
}

fn hello() -> String {
    "Hello, user!".to_string()
}

fn date() -> String {
    format!("Today's date is {}", chrono::Local::now().format("%Y-%m-%d"))
}

fn time() -> String {
    format!("The current time is {}", chrono::Local::now().format("%H:%M:%S"))
}

fn clear() -> String {
    // ANSI escape sequence for clearing screen (optional for terminal UI)
    "\x1B[2J\x1B[1;1H".to_string()
}

fn pwd() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => format!("Error retrieving current directory: {}", e),
    }
}

fn ls() -> String {
    match fs::read_dir(".") {
        Ok(entries) => {
            let files: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .collect();
            files.join("\n")
        }
        Err(e) => format!("Error listing files: {}", e),
    }
}

fn whoami() -> String {
    match env::var("USER").or_else(|_| env::var("USERNAME")) {
        Ok(user) => format!("Current user: {}", user),
        Err(_) => "Could not retrieve username".to_string(),
    }
}

fn help() -> String {
    vec![
        "Available commands:",
        "  hello        - Greets the user.",
        "  date         - Shows the current date.",
        "  time         - Shows the current time.",
        "  clear        - Clears the screen.",
        "  pwd          - Prints the current working directory.",
        "  ls           - Lists files in the current directory.",
        "  whoami       - Displays the current username.",
        "  echo [msg]   - Displays the message entered.",
        "  cat [file]   - Displays the contents of a file.",
        "  touch [file] - Creates an empty file.",
        "  rm [file]    - Deletes the specified file.",
        "  write [file] \"<text>\" - Appends text to a file.",
        "  cd [dir]     - Changes the current directory.",
        "  exit         - Exits the terminal.",
    ]
    .join("\n")
}

fn echo(message: &str) -> String {
    message.to_string()
}

fn cat(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => format!("Error reading file: {}", e),
    }
}

fn touch(filename: &str) -> String {
    match fs::File::create(filename) {
        Ok(_) => format!("Created file: {}", filename),
        Err(e) => format!("Error creating file: {}", e),
    }
}

fn rm(filename: &str) -> String {
    match fs::remove_file(filename) {
        Ok(_) => format!("Deleted file: {}", filename),
        Err(e) => format!("Error deleting file: {}", e),
    }
}

fn cd(path: &str) -> String {
    match env::set_current_dir(path) {
        Ok(_) => format!("Changed directory to: {}", path),
        Err(e) => format!("Error changing directory: {}", e),
    }
}

fn write(args: &str) -> String {
    let mut parts = args.splitn(2, ' ');
    let filename = parts.next().unwrap_or("");
    let content = parts.next().unwrap_or("");

    if filename.is_empty() || content.is_empty() {
        return "Usage: write <filename> \"<text>\"".to_string();
    }

    match OpenOptions::new().create(true).append(true).open(filename) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", content) {
                format!("Error writing to file: {}", e)
            } else {
                format!("Text written to {}", filename)
            }
        }
        Err(e) => format!("Error opening file: {}", e),
    }
}
