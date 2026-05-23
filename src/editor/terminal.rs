use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};
use crate::editor::terminal::ClearLineDirection::Right;
// Write is a Trait used to write to a stream.

pub enum ClearLineDirection {
    Right,
    Left,
    All,
}
#[derive(Copy, Clone)]  // Copy and Clone are used to make a copy of a struct. - Traits
pub struct Size {
    pub height: u16,
    pub width: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x: 0, y: 0})?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::UntilNewLine))?;
        Ok(())
    }
    pub fn clear_line(direction: ClearLineDirection) -> Result<(), Error> {
        match direction {
            ClearLineDirection::Right => {
                queue!(stdout(), Clear(ClearType::UntilNewLine))?;
            }
            ClearLineDirection::Left => {
                queue!(stdout(), Print("\x1b[1K"))?;
            }
            ClearLineDirection::All => {
                queue!(stdout(), Clear(ClearType::CurrentLine))?;
            }
        }
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn display_editor_title() -> Result<(), Error> {
        let size = Self::size()?;
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let message = format!("{name} -- version {version}");
        Self::move_cursor_to(Position{
            x: (size.width/2u16) - (message.len() as u16)/2,
            y: size.height/ 3u16
        })?;
        Self::clear_line(Right)?;
        Self::print(&message)?;
        Ok(())
    }
}