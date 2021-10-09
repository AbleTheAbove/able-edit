#![warn(missing_docs)]
///

///
fn main() {
    let mut editor = editor::Editor::default();

    editor.run();
}
/// Editor module
mod editor;
/// Terminal module
mod terminal;
/// The theme module used to style the editor
pub mod theme;
