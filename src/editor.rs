use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Size, Position};
use crate::editor::terminal::ClearLineDirection::{All, Right};

pub struct Editor {
    should_quit: bool,
}

impl Editor {

    /*
     * default implementation of editor
     * - empty brackets mean that function takes no arguments and it can be
     * used as a static method
     * - -> Self means that the function returns an instance of struct it is implemented on, in
     * this case, an Editor instance
     */
    //const here means that the function will be evaluated at compile time, not runtime
    pub const fn default() -> Self {
        // In Rust: the last line of a block is returned if it does not end with a semicolon
        // Here the last line returns an Editor instance, empty
        // there can be Editor or Self
        Self { should_quit: false }
    }

    // mut indicated that we will be modifying the reference
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    /*Self is the same as this in java
     &self means that the function takes a reference to an instance of struct it is implemented on,
     in this case, an Editor instance
     -> Result<(), std::io::Error> means that the function returns a Result type,
     either Ok with nothing or Err with a std::io::Error in it
     */

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())

    }
    fn evaluate_event(&mut self, event: &Event) {
        // the ".." tells rust to ignore the rest of the fields in the KeyEvent struct
        if let Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                // we are operating on a reference to an event, so (*)Modifiers
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }


    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::display_editor_title()?;
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line(All)?;
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
