use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    // Self is the same as this in java
    // &self means that the function takes a reference to an instance of struct it is implemented on,
    // in this case, an Editor instance
    // -> Result<(), std::io::Error> means that the function returns a Result type,
    //  either Ok with nothing or Err with a std::io::Error in it
    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        // ? unwraps the result, returning the error if it is Err, or continuing if it is Ok
        enable_raw_mode()?;
        // loop continues until break is encountered
        loop {
            // we take keyevent and destructure it into its fields
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    // we are only matching if further the modifiers are control, so 'q' is only quit if ctrl-q is pressed
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        // Our function now needs to return something.
        // We leverage the fact that Rust treats the last line as something we want to return (notice the missing ;).
        // If we made it down here, it means none of the functions with a ? have found an error that has been returned, so we can safely return a pink box labeled Ok with nothing in it.
        // The code representation of this is: Ok(()).
        Ok(())
    }
}
