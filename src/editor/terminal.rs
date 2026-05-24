use core::fmt::Display;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};
// Write is a Trait used to write to a stream.

#[derive(Copy, Clone)] // Copy and Clone are used to make a copy of a struct. - Traits
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
// '///' doc comments are used to document the struct.

///Represents the Terminal.
///Edge case for platforms where 'usize' < 'u16'
///Regardless of the actual size of the Terminal, representation only spans
///over at most 'usize::MAX' or 'u16::size' rows/columns, whichever is smaller.
///Each size returned truncates to min('usize::MAX', 'u16::MAX')
///And should you attempt to set the cursor out of these bounds, it will be clamped to the nearest valid position.

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
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    /// Moves the cursor to the fiven Position
    /// # Arguments
    ///  * 'Position' - the 'position' to move the cursor to. Will be truncated to 'u16::MAX' if out of bounds.
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        // clipy::as_conversions:: se doc above
        #[allow(clipy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    // This is one way of requesting that whatever is passed to this method should implement the Display trait.
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        // clippy::as_conversions: see doc above
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    // this tells rust that this method is generic over any type that implements the Command trait
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
