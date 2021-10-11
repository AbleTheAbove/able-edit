//! Terminal module
//! Used for implementing the terminal functionality of able-edit

///
pub struct Terminal {
    pub height: u16,
    pub focused: bool,
}
///
impl Terminal {
    ///
    pub fn default() -> Terminal {
        Terminal {
            height: 2,
            focused: false,
        }
    }

    pub fn run() {}
}
