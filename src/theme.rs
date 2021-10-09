/// A structure that holds all of the colors for a theme
pub struct Theme {
    ///
    pub foreground: AnsiValue,
    ///
    pub background: AnsiValue,
    ///
    pub math_operators: AnsiValue,
    ///
    pub keywords: AnsiValue,
    ///
    pub outline: AnsiValue,
    ///
    pub numbers: AnsiValue,
    ///
    pub string_literals: AnsiValue,
    ///
    pub comments: AnsiValue,
}

impl Theme {
    /// The default dark theme which is liable to change
    pub fn default_dark() -> Theme {
        let theme = Theme {
            foreground: AnsiValue::rgb(255, 255, 255),
            background: AnsiValue::rgb(255, 255, 255),
            math_operators: AnsiValue::rgb(0, 0, 0),
            keywords: AnsiValue::rgb(0, 0, 0),
            outline: AnsiValue::rgb(0, 0, 0),
            numbers: AnsiValue::rgb(0, 0, 0),
            string_literals: AnsiValue::rgb(0, 0, 0),
            comments: AnsiValue::rgb(0, 0, 0),
        };
        theme
    }
}

use termion::color::AnsiValue;
