use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, Terminal, widgets::*};
use log::{debug, error};

use crate::game;

#[derive(Debug)]
pub struct TerminalData {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalData {
    pub fn new() -> Result<Self, std::io::Error> {
        debug!("TerminalData::new() - Creating new TerminalData.");
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn cleanup_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("restore_terminal: Entering function.");
        let mut error_occurred: Option<Box<dyn Error>> = None;

        match disable_raw_mode() {
            Ok(_) => debug!("raw mode enabled."),
            Err(error) => {
                error!("disabling of raw mode failed: {}", error);
                error_occurred = Some(Box::new(error));
            }
        }

        match execute!(self.terminal.backend_mut(), LeaveAlternateScreen) {
            Ok(_) => debug!("execution of LeaveAlternateScreen succeeded."),
            Err(error) => {
                error!("execution of LeaveAlternateScreen failed: {}", error);
                if error_occurred.is_none() { error_occurred = Some(Box::new(error)); }
            }
        }

        match self.terminal.show_cursor() {
            Ok(_) => debug!("show_cursor success."),
            Err(error) => {
                error!("execution of show_cursor failed: {}", error);
                if error_occurred.is_none() { error_occurred = Some(Box::new(error)); }
            }
        }

        match error_occurred {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

    pub fn render_frame(frame: &mut Frame, app_state: &game::AppState) {
        let size = frame.size();

        let block = Block::default().title("Ratatui Type Racer TUI")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        match app_state .status {
            game::GameStatus::NotStarted => {
                let user = whoami::username();

                let paragraph = Paragraph::new(format!("Hello {}! Press 'ESC' to quit.", user))
                    .style(Style::default().fg(Color::Blue))
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, size);
            }
            game::GameStatus::InProgress => {
                let paragraph = Paragraph::new(format!("Testing your typing speed..."))
                    .style(Style::default().fg(Color::Red))
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, size);
            }
            game::GameStatus::Finished => {
                // Draw the target text
            }
            game::GameStatus::Exiting => {
                let user = whoami::username();

                let paragraph = Paragraph::new(format!("Hello {}! Press 'ESC' to quit.", user))
                    .style(Style::default().fg(Color::Blue))
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, size);
            }
        }
    }
}