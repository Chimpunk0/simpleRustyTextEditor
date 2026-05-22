use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

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
        Editor {}
    }

    // Self is the same as this in java
    // &self means that the function takes a reference to an instance of struct it is implemented on,
    // in this case, an Editor instance
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        // loop continues until break is encountered
        loop {
            match read() {
                // this says, if read returns OK and inside is a Key, process event in the "Key box"
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match (event.code) {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        // this says "do nothing" () for any other ("_") key
                        _ => (),
                    }
                }
                // {:?} prints the debug representation of the error
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }

        disable_raw_mode().unwrap();
    }
}
