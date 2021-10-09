fn main() {
    let mut editor = Editor::default();

    editor.run();
}

mod editor;
mod terminal;
pub mod theme;
use editor::Editor;
