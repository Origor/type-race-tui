use std::time::Instant;
use crossterm::event::KeyCode;
use log::{debug, info};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Finished,
    Exiting,
}

#[derive(Debug)]
pub struct AppState {
    pub target_text: String,
    // pub target_text_chunk: Vec<char>,
    pub typed_chars: Vec<char>,
    pub current_index: usize,
    pub status: GameStatus,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub mistakes: usize,
    // pub wpm: u32, // Consider adding calculated fields if needed often
    // pub accuracy: f32,
}

impl AppState {
    pub fn new(text_to_type: String) -> Self {
        debug!("AppState::new() - Creating new AppState.");
        AppState {
            target_text: text_to_type,
            typed_chars: Vec::new(),
            current_index: 0,
            status: GameStatus::NotStarted,
            start_time: None,
            end_time: None,
            mistakes: 0,
        }
    }

    pub fn handle_keypress(&mut self, key_code: KeyCode) {
        if self.status == GameStatus::Finished {
            debug!("AppState::handle_keypress() - Keypress ignored: Game already finished.");
            return;
        }

        match key_code {
            KeyCode::Char(pressed_character) => {
                if self.status == GameStatus::NotStarted {
                    self.status = GameStatus::InProgress;
                    self.start_time = Some(Instant::now());
                    debug!("Game status -> InProgress, Timer started.");
                }

                if self.status == GameStatus::InProgress {
                    if self.current_index < self.target_text.chars().count() {
                        if Some(pressed_character) != self.target_text.chars().nth(self.current_index) {
                            self.mistakes += 1;
                            debug!("Mistake registered. Total: {}", self.mistakes);
                        }

                        self.typed_chars.push(pressed_character);
                        self.current_index += 1;
                        debug!("Character '{}' processed. Index: {}", pressed_character, self.current_index);

                        if self.current_index == self.target_text.chars().count() {
                            self.status = GameStatus::Finished;
                            self.end_time = Some(Instant::now());
                            info!("Game status -> Finished.");
                            // TODO: Calculate and store final WPM/Accuracy here?
                        }
                    }
                    else {
                        debug!("Input '{}' ignored: End of target text reached.", pressed_character);
                    }
                }
            }

            KeyCode::Backspace => {
                if self.status == GameStatus::InProgress && self.current_index > 0 {
                    self.current_index -= 1;
                    let _removed_char = self.typed_chars.pop();
                    debug!("Backspace processed. Index: {}", self.current_index);
                }
                else {
                    debug!("Backspace ignored: No character to delete or game not in progress.");
                }
            }

            KeyCode::Esc => {
                self.cancel();
            }

            _ => {
                debug!("Unhandled key press: {:?}", key_code);
            }
        }
    }

    pub fn calculate_wpm(&self) -> u32 {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let duration = end.duration_since(start);
            let minutes = duration.as_secs_f64() / 60.0;

            // Standard WPM calculation uses 5 characters per word
            let num_chars = self.target_text.chars().count();
            if minutes > 0.0 && num_chars > 0 {
                let num_words = num_chars as f64 / 5.0;
                return (num_words / minutes).round() as u32;
            }
        }
        0 // Return 0 if game not finished or duration is zero
    }

    pub fn calculate_accuracy(&self) -> f32 {
        if self.current_index == 0 { 100.0 }
        else {
            // Accuracy = (Typed Chars - Mistakes) / Typed Chars * 100
            let correct_chars = self.current_index.saturating_sub(self.mistakes);
            let accuracy = (correct_chars as f32 / self.current_index as f32) * 100.0;
            accuracy.max(0.0).min(100.0)
        }
    }

    pub fn reset(&mut self) {
        self.typed_chars.clear();
        self.current_index = 0;
        self.status = GameStatus::NotStarted;
        self.start_time = None;
        self.end_time = None;
        self.mistakes = 0;
        info!("Game state reset.");
    }

    pub fn cancel(&mut self) {
        self.status = GameStatus::Exiting;
        self.end_time = Some(Instant::now());
        info!("Game Exiting. Total mistakes: {}", self.mistakes);
    }
}

