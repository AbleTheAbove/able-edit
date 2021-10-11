pub struct Color(u8, u8, u8);
impl Color {
    pub fn to_ascii(&self) -> String {
        AnsiValue::rgb(self.0, self.1, self.2).fg_string()
    }
}

/// A structure that holds all of the colors for a theme
#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    ///
    pub foreground: Option<[u8; 3]>,
    ///
    pub background: Option<[u8; 3]>,
    ///
    pub math_operators: Option<[u8; 3]>,
    ///
    pub keywords: Option<[u8; 3]>,
    ///
    pub outline: Option<[u8; 3]>,
    ///
    pub numbers: Option<[u8; 3]>,
    ///
    pub string_literals: Option<[u8; 3]>,
    ///
    pub comments: Option<[u8; 3]>,
}

impl Theme {
    /// The default dark theme which is liable to change
    pub fn default_dark() -> Theme {
        let theme = Theme {
            foreground: Some([255, 255, 255]),
            background: Some([0, 0, 0]),
            math_operators: Some([0, 0, 0]),
            keywords: Some([0, 0, 0]),
            outline: Some([0, 0, 0]),
            numbers: Some([0, 0, 0]),
            string_literals: Some([0, 0, 0]),
            comments: Some([0, 0, 0]),
        };
        theme
    }
}

use serde_derive::{Deserialize, Serialize};
use termion::color::AnsiValue;
