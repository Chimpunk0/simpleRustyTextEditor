use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::terminal::size;
use crossterm::cursor::MoveTo;
use std::io::stdout;

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
    pub fn default() -> Self {
        // In Rust: the last line of a block is returned if it does not end with a semicolon
        // Here the last line returns an Editor instance, empty
        Editor { should_quit: false }
    }

    // mut indicated that we will be modifying the reference
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    /*Self is the same as this in java
     &self means that the function takes a reference to an instance of struct it is implemented on,
     in this case, an Editor instance
     -> Result<(), std::io::Error> means that the function returns a Result type,
     either Ok with nothing or Err with a std::io::Error in it
     */
    fn initialize() -> Result<(), std::io::Error> {
        // ? unwraps the result, returning the error if it is Err, or continuing if it is Ok
        enable_raw_mode()?;
        Self::clear_screen()

    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        /*
        execute! macro is a macro that executes a function on the stdout object.
         */
        execute!(stdout, Clear(ClearType::All))
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            println!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let term_size = size();
        println!("{:?}", term_size);
    }
}
