//we tell clippy which lints to enable by defaultf
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
    // now we are running the run() function output of default(), which is an Editor instance
    Editor::new().unwrap().run();
}
