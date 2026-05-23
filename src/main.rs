#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)] //we tell clippy which lints to enable by default
mod editor;
use editor::Editor;

fn main() {
    // now we are running the run() function output of default(), which is an Editor instance
    Editor::default().run();
}
