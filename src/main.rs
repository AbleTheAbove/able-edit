#![warn(missing_docs)]
//! Able-Edit is a text/code editor built specifically for me.

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
