use std::{
    error::Error,
    io::{self},
    time::Duration,
};

use crossterm::event::{self, Event};
use log::{debug, error, info};

mod logging;
mod tui;
mod game;

fn run_app(target_text: String) -> Result<(), Box<dyn Error>> {
    debug!("run_app: Entering function.");

    let mut terminal_data = tui::TerminalData::new()?;
    let mut app_state = game::AppState::new(target_text);

    loop {
        terminal_data.terminal.draw(|frame| { tui::TerminalData::render_frame(frame, &mut app_state); })?;

        if app_state.status == game::GameStatus::Finished {
            debug!("run_app: Game finished, exiting loop.");
            terminal_data.cleanup_terminal()?;
            return Ok(());
        }

        if event::poll(Duration::from_millis(250)).is_err() {
            error!("run_app: Error polling for events.");
            terminal_data.cleanup_terminal()?;
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Error polling for events",
            )));
        }
        else {
            let event = match event::read() {
                Ok(event) => event,
                Err(error) => {
                    error!("run_app: Error reading event: {}", error);
                    return Err(Box::new(error));
                }
            };
            if let Event::Key(key) = event {
                app_state.handle_keypress(key.code);
            } else {
                // Timeout elapsed, no event, loop continues
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = logging::setup_logging() {
        eprintln!("CRITICAL: Failed to set up logging: {}", e);
        return Err(e);
    }
    info!("Initializing application...");

    let target_text = String::from("Example text...");

    let app_result = run_app(target_text);
    if let Err(err) = app_result {
        error!("Error during app execution: {:?}", err);
    }

    info!("Exiting application...");
    Ok(())
}
