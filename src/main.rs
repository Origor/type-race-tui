use std::{
    error::Error,
    fs::File,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use log::{debug, error, info, LevelFilter};
use simplelog::{Config, WriteLogger};
use ratatui::{prelude::*, widgets::*};

fn setup_logging() -> Result<(), Box<dyn Error>> {
    debug!("setup_logging: Entering function.");
    let log_file_result = File::create("/app/logs/app.log");

    match log_file_result {
        Ok(log_file) => {
            if WriteLogger::init(LevelFilter::Debug, Config::default(), log_file).is_err() {
                error!("ERROR: Failed to initialize file logger!");
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to initialize file logger",
                )));
            } else {
                info!("Logging initialized successfully.");
                return Ok(());
            }
        },
        Err(error) => {
            error!("ERROR: Failed to create log file '/app/logs/app.log': {}", error);
            return Err(Box::new(error));
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    debug!("setup_terminal: Entering function.");
    let mut stdout = io::stdout();

    match enable_raw_mode() {
        Ok(_) => debug!("setup_terminal: raw mode enabled."),
        Err(error) => error!("setup_terminal: enabling of raw mode failed: {}", error),
    }

    match execute!(stdout, EnterAlternateScreen) {
        Ok(_) => debug!("setup_terminal: EnterAlternateScreen enabled."),
        Err(error) => error!("setup_terminal: execution of EnterAlternateScreen failed: {}", error),
    }

    let backend = CrosstermBackend::new(stdout);

    let terminal = Terminal::new(backend);
    match terminal {
        Ok(terminal) => {
            debug!("setup_terminal: Terminal created successfully.");
            Ok(terminal)
        }
        Err(error) => {
            error!("setup_terminal: Terminal creation failed: {}", error);
            Err(Box::new(error))
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    debug!("run_app: Entering function.");

    loop {
        match terminal.draw(|frame| {
                let size = frame.size();
                let block = Block::default().title("Ratatui Type Racer TUI")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green));
                let user = whoami::username();
                let paragraph = Paragraph::new(format!("Hello {}! Press 'q' to quit.", user))
                    .style(Style::default().fg(Color::Blue))
                    .block(block)
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, size);
            })
        {
            Ok(_) => debug!("run_app: Terminal drawn successfully."),
            Err(error) => {
                error!("run_app: Error drawing terminal: {}", error);
                return Err(error);
            }
        }

        if event::poll(Duration::from_millis(250)).is_err() {
            error!("run_app: Error polling for events.");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error polling for events",
            ));
        }
        else {
            let event = match event::read() {
                Ok(event) => event,
                Err(error) => {
                    error!("run_app: Error reading event: {}", error);
                    return Err(error);
                }
            };
            if let Event::Key(key) = event {
                if key.code == KeyCode::Char('q') {
                    debug!("run_app: 'q' pressed, exiting loop.");
                    return Ok(());
                }
            } else {
                // Timeout elapsed, no event, loop continues
            }
        }
    }
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>, ) -> Result<(), Box<dyn Error>> {
    debug!("restore_terminal: Entering function.");

    match disable_raw_mode() {
        Ok(_) => debug!("restore_terminal: raw mode enabled."),
        Err(error) => error!("restore_terminal: enabling of raw mode failed: {}", error),
    }

    match execute!(terminal.backend_mut(), LeaveAlternateScreen) {
        Ok(_) => debug!("restore_terminal: LeaveAlternateScreen success."),
        Err(error) => error!("restore_terminal: execution of LeaveAlternateScreen failed: {}", error),
    }

    match terminal.show_cursor() {
        Ok(_) => {
            debug!("restore_terminal: show_cursor success.");
            Ok(())
        },
        Err(error) => {
            error!("restore_terminal: execution of show_cursor failed: {}", error);
            Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to restore terminal")))
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    info!("Starting application...");
    match setup_logging() {
        Ok(_) => (),
        Err(error) => { return Err(error); }
    }

    let mut terminal = match setup_terminal() {
        Ok(terminal) => terminal,
        Err(error) => return Err(error),
    };

    let app_result = run_app(&mut terminal);
    if let Err(err) = app_result {
        error!("Error during app execution: {:?}", err);
    }

    let restore_result = restore_terminal(&mut terminal);
    if let Err(err) = restore_result {
        error!("Error during terminal restore: {:?}", err);
        return Err(err);
    }

    info!("Exiting application...");
    Ok(())
}
