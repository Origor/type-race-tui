## TUI TypeRacer Implementation Plan

Here's a breakdown of the steps and components involved in creating the typing game:

**1. Game State Management (Rust)**

You'll need a way to keep track of everything happening in the game. A struct is perfect for this.

* **```Create a struct (e.g., AppState or Game)```**: This struct will hold the current state, including:
  * `target_text: String`: The full text passage to be typed.
  * `typed_chars: Vec<char>`: A vector storing the characters the user has actually typed so far. This makes handling backspace and comparing character-by-character easier than using a `String`.
  * `current_index: usize`: The index within `target_text` representing the character the user is *about* to type (where the "cursor" is).
  * `status: GameStatus`: An enum to track the game phase (e.g., `NotStarted`, `InProgress`, `Finished`).
  * `start_time: Option<std::time::Instant>`: When the typing started (set on the first valid keypress).
  * `end_time: Option<std::time::Instant>`: When the typing finished.
  * `mistakes: usize`: A counter for incorrect keypresses. (You might refine how mistakes are counted later).

**2. Loading the Text**

* **Start Simple:** Initially, hardcode a `target_text` string directly in your code.
* **Later:** Implement logic to load text passages from files (e.g., text files included in your Docker image or mounted via volumes).

**3. Handling User Input (Rust + Crossterm/Ratatui)**

* **Event Loop:** Use the existing event loop structure (`event::poll`, `event::read`) from the example code.
* **Process Key Events:** Inside the loop, when you receive `Ok(Event::Key(key))`:
  * Check `key.code`:
    * If `KeyCode::Char(c)`:
      * If `status == GameStatus::NotStarted`, set `start_time` and change `status` to `InProgress`.
      * If `status == GameStatus::InProgress`:
        * Append `c` to `typed_chars`.
        * Compare `c` with `target_text.chars().nth(current_index)`. Increment `mistakes` if they don't match.
        * Increment `current_index`.
        * Check if `current_index` has reached the end of `target_text`. If so, set `end_time` and change `status` to `Finished`.
    * If `KeyCode::Backspace`:
      * If `current_index > 0`:
        * Decrement `current_index`.
        * Check if the character *`at the new current_index`* in `typed_chars` was a mistake compared to `target_text`. If so, potentially decrement `mistakes` (optional, depends on your rules).
        * Remove the last character from `typed_chars`.
    * If `KeyCode::Esc` (or 'q'): Allow quitting.
* **Update State:** Modify the `AppState` struct based on the input.

**4. Rendering the UI (Rust + Ratatui)**

* **`terminal.draw()`** Closure: This is where you'll translate your `AppState` into visuals.
* **`Paragraph`** Widget: This is ideal for displaying the text.
* **`Styled Spans (ratatui::text::Span):`** The key is to build the text line by line (or span by span) with different styles:
  * Iterate through the `target_text` characters up to `current_index` (or maybe a bit beyond).
  * For each character `i`:
    * If `i < typed_chars.len()`: Compare `typed_chars[i]` with `target_text[i]`.
      * If match: Create a `Span` with `Style::default().fg(Color::Green)` (or Gray, etc.).
      * If mismatch: Create a `Span` with `Style::default().bg(Color::Red)`.
    * If `i == current_index`: This is the cursor position. Style the character `target_text[i]` with `Style::default().add_modifier(Modifier::REVERSED)` or a unique background color.
    * If `i > current_index`: These are upcoming characters. Style them with `Style::default().fg(Color::DarkGray)` (or similar).
  * Combine these `Span`s into `Line`s and pass them to `Paragraph::new()`.
* **Displaying Stats:** Use another `Paragraph` or `Table` widget placed elsewhere (e.g., below the main text) to display:
  * Time elapsed (calculate from `start_time`).
  * WPM (calculate based on correctly typed characters / time).
  * Accuracy (calculate based on correct characters / total typed characters or `current_index`).
  * Mistakes count.

**5. Game Loop Logic (Rust + Ratatui)**

* **Initialization:** Create the initial `AppState`.
* **Loop:** The `run_app` function contains the main loop.
  * Poll for events.
  * Handle input and update `AppState`.
  * Call `terminal.draw()` to render the current `AppState`.
  * Check `AppState.status` to see if the game finished. If so, maybe display final results and wait for a keypress to exit or restart.

**6. Calculations (Rust)**

* Implement helper functions:
  * `calculate_wpm(correct_chars: usize, duration: Duration) -> u32`
  * `calculate_accuracy(correct_chars: usize, total_typed: usize) -> f32`
  * Remember the standard definition of a "word" for WPM is often 5 characters.

**7. Logging (Rust + Log/Simplelog)**

* Use `debug!`, `info!`, `warn!`, `error!` macros throughout your input handling, state updates, and rendering logic to trace execution flow and variable values. Write these logs to your `app.log` file (viewable via `docker cp` or volume mounts).

**Iteration:**

* Start with the basics: display text, handle character input, show simple correct/incorrect feedback.
* Then add the cursor, backspace handling, timer, WPM/accuracy calculations, and finally the results display.

This structured approach, using `AppState` to manage state and Ratatui widgets for rendering, combined with file logging for debugging, should allow you to build your TUI TypeRacer effectively within the Docker environment.