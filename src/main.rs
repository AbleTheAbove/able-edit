#![warn(missing_docs)]
//! Able-Edit is a text/code editor built specifically for me.

///
fn main() {
    let mut editor = editor::Editor::default();
    plugins::Plugin::test();

    editor.run();
}
/// Editor module
mod editor;
/// Plugin system that will be incorperated fully as time goes on
mod plugins;
/// Terminal module
mod terminal;
/// The theme module used to style the editor
pub mod theme;
