#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
mod status_message;

use editor::Editor;
pub use document::Document;
pub use row::Row;
pub use terminal::Terminal;
pub use editor::Position;
pub use status_message::StatusMessage;

fn main() {
    Editor::default().run();
}

