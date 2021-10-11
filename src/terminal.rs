//! Terminal module
//! Used for implementing the terminal functionality of able-edit

///
pub struct Terminal {
    pub height: u16,
    pub focused: bool,
}
///
impl Terminal {
    /// provide a default for terminal
    pub fn default() -> Self {
        Self {
            height: 2,
            focused: false,
        }
    }
}
