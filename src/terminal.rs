pub struct Terminal {
    pub height: u16,
    pub focused: bool,
}

impl Terminal {
    pub fn default() -> Terminal {
        Terminal {
            height: 2,
            focused: false,
        }
    }
}
