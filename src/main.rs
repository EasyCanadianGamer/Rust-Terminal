mod commands;

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the application
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut input = String::new(); // Stores user input
    let mut output = Vec::new();  // Stores command results

    loop {
        // Draw the UI
        terminal.draw(|f| {
            // Layout with two vertical sections
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Min(5), // Output area
                        Constraint::Length(3), // Input area
                    ]
                    .as_ref(),
                )
                .split(f.area()); // Use `f.area()` instead of `f.size()`

            // Output area
            let output_text: Vec<Line> = output
                .iter()
                .map(|line| Line::from(line.clone()))
                .collect();
            let output_widget = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Output"));
            f.render_widget(output_widget, chunks[0]);

            // Input area
            let input_widget = Paragraph::new(input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input_widget, chunks[1]);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input.push(c), // Add character to input
                    KeyCode::Backspace => {
                        input.pop(); // Remove last character
                    }
                    KeyCode::Enter => {
                        // Process the input
                        if input.trim() == "exit" {
                            return Ok(()); // Exit the application
                        }
                        let result = process_command(input.trim());
                        output.push(format!("> {}", input));
                        output.push(result);
                        input.clear();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn process_command(input: &str) -> String {
    let mut result = Vec::new();
    commands::execute_command(input);
    result.push(format!("Executed: {}", input));
    result.join("\n")
}
