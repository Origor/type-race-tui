use std::{
    error::Error,
    fs::File, // Needed for file logging
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, error, info, LevelFilter}; // Import log macros and LevelFilter
use ratatui::{prelude::*, widgets::*};
use simplelog::{Config, WriteLogger}; // Import simplelog components

// Helper function to setup the terminal
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    debug!("DEBUG: setup_terminal: Entering function.");
    let mut stdout = io::stdout();
    debug!("DEBUG: setup_terminal: Enabling raw mode...");
    enable_raw_mode()?;
    debug!("DEBUG: setup_terminal: Entering alternate screen...");
    execute!(stdout, EnterAlternateScreen)?;
    debug!("DEBUG: setup_terminal: Creating terminal backend...");
    let backend = CrosstermBackend::new(stdout);
    debug!("DEBUG: setup_terminal: Creating terminal...");
    let terminal = Terminal::new(backend)?;
    debug!("DEBUG: setup_terminal: Setup complete.");
    Ok(terminal)
}

// Helper function to restore the terminal
fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    debug!("DEBUG: restore_terminal: Entering function.");
    debug!("DEBUG: restore_terminal: Disabling raw mode...");
    disable_raw_mode()?;
    debug!("DEBUG: restore_terminal: Leaving alternate screen...");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    debug!("DEBUG: restore_terminal: Showing cursor...");
    terminal.show_cursor()?;
    debug!("DEBUG: restore_terminal: Restore complete.");
    Ok(())
}

// Main application function
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    debug!("DEBUG: run_app: Entering function.");
    loop {

        // Draw the UI
        terminal.draw(|f| {
            let size = f.size();
            // Create a simple paragraph widget
            let block = Block::default().title("Ratatui Docker Example").borders(Borders::ALL);
            let paragraph = Paragraph::new("Hello Ratatui! Press 'q' to quit.")
                .style(Style::default().fg(Color::Yellow))
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, size);
        })?;

        // Handle input events
        if event::poll(Duration::from_millis(250))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.code == KeyCode::Char('q') {
                    debug!("DEBUG: run_app: 'q' pressed, exiting loop.");
                    return Ok(()); // Quit if 'q' is pressed
                }
            }
        } else {
             // Timeout elapsed, no event, loop continues
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // --- Initialize Logging ---
    // Attempt to create or open the log file.
    // Using .unwrap_or_else() provides a fallback if init fails,
    // preventing the app from crashing just because logging isn't set up.
    let log_file_result = File::create("/app/logs/app.log");
    match log_file_result {
        Ok(log_file) => {
            // Initialize the WriteLogger. Change LevelFilter to Info, Warn, Error etc. as needed.
            if WriteLogger::init(LevelFilter::Debug, Config::default(), log_file).is_err() {
                // Use eprintln! only if logger setup fails
                eprintln!("ERROR: Failed to initialize file logger!");
            } else {
                // Optional: Log that logging started successfully (can be seen in the file)
                info!("File logger initialized.");
            }
        },
        Err(e) => {
             eprintln!("ERROR: Failed to create log file '/app/logs/app.log': {}", e);
             // Consider adding a fallback TermLogger here if file logging is critical
        }
    }
    // --- End Logging Initialization ---
    // Use println here as terminal is not yet controlled
    println!("DEBUG: main: Program start.");

    // Use eprintln for steps around terminal control
    debug!("DEBUG: main: Attempting to set up terminal...");
    let mut terminal = setup_terminal()?;
    debug!("DEBUG: main: Terminal setup successful.");

    debug!("DEBUG: main: Entering run_app...");
    let app_result = run_app(&mut terminal);
    debug!("DEBUG: main: run_app finished. Result: {:?}", app_result);


    debug!("DEBUG: main: Attempting to restore terminal...");
    // Pass terminal by value to consume it if restore fails early? No, mutable ref is fine.
    let restore_result = restore_terminal(&mut terminal);
    debug!("DEBUG: main: Terminal restore finished. Result: {:?}", restore_result);


    debug!("DEBUG: main: Handling potential app error...");
    if let Err(err) = app_result {
        error!("Error during app execution: {:?}", err); // Use error! for errors
        // Potentially return the error or handle it
        // return Err(Box::new(err)); // Convert io::Error to Box<dyn Error> if needed
    }

    if let Err(err) = restore_result {
        error!("Error during terminal restore: {:?}", err); // Use error!
        return Err(err); // Return the restore error
    }

    // Use println here as terminal is restored
    println!("DEBUG: main: Program end.");
    Ok(())
}
