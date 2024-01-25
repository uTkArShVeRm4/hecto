#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
fn main() {
    let mut editor = Editor::default();
    editor.run();
}
