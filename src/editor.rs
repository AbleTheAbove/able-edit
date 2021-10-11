//!

/// The error handling function
fn error_handle(e: std::io::Error) {
    println!("{}", e);
}
///
pub struct Editor {
    pub theme: Theme,
    pub cursor: (u16, u16),
    pub text: String,
    pub path: Option<PathBuf>,
    pub terminal: crate::terminal::Terminal,
    pub focused: bool,
}
///
impl Editor {
    pub fn run(&mut self) {
        self.init();

        let mut stdout = stdout().into_raw_mode().unwrap();
        let size = terminal_size().unwrap();

        self.render();
        println!(
            "{}Welcome to able-edit.",
            termion::cursor::Goto(2, size.1 - 1)
        );
        stdout.flush().unwrap();

        for key in stdin().keys() {
            if self.focused {
                self.render();
                print!("{}", termion::cursor::Hide);
                print!("{}", termion::cursor::Goto(1, 2));

                if let Ok(key) = key {
                    print!("{}{:?}", termion::cursor::Goto(2, size.0), key);
                };

                match key {
                    Ok(key) => match key {
                        Key::Char('\n') => {
                            self.text.push('\n');
                        }
                        Key::Char(c) => {
                            if c.is_control() {
                            } else {
                                self.text.push(c);
                            }
                        }
                        Key::Ctrl('t') => self.terminal.focused = !self.terminal.focused,
                        Key::Ctrl('q') => break,
                        Key::Backspace => {
                            self.text.pop();
                        }
                        _ => {
                            print!("{}{:?}", termion::cursor::Goto(2, size.0), key);
                        }
                    },
                    Err(err) => error_handle(err),
                }
                let mut x_it = 0;
                let mut y_it = 0;

                pub fn text_render(x_it: u16, y_it: u16) {
                    print!(
                        "{}{}",
                        termion::cursor::Goto(x_it + 2, y_it + 2),
                        termion::cursor::Show
                    );
                }

                for x in self.text.chars() {
                    let editor_height = Editor::size().0 - self.terminal.height;

                    if y_it > editor_height {
                    } else {
                        text_render(x_it, y_it);
                        if x == '\n' {
                            x_it = 0;
                            y_it += 1;
                            text_render(x_it, y_it);
                        } else {
                            print!("{}", x);
                            x_it += 1;
                        }
                    }
                }
                stdout.flush().unwrap();
            } else {
                match key {
                    Ok(key) => match key {
                        Key::Ctrl('q') => break,
                        Key::Ctrl('s') => self.save(),
                        Key::Ctrl('t') => self.focused = !self.focused,
                        _ => {}
                    },
                    Err(err) => error_handle(err),
                }
            }
        }

        Editor::clear();
    }

    fn init(&self) {
        let filename = "config.toml";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        #[derive(Debug, Deserialize)]
        struct Config {
            pub theme: Option<Theme>,
        }

        let decoded: Config = toml::from_str(&contents).unwrap();
        // println!("{:#?}", decoded);

        // println!("With text:\n{}", contents);
    }
    pub fn default() -> Self {
        Self {
            theme: Theme::default_dark(),
            cursor: (0, 0),
            text: "".to_string(),
            path: None,
            terminal: Terminal::default(),
            focused: false,
        }
    }
}
/// Drawing API
impl Editor {
    pub fn render(&self) {
        Editor::clear();
        self.draw_outline();
    }
    pub fn draw_outline(&self) {
        // │ ┤ ┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌
        let size = terminal_size().unwrap();
        print!("{}", termion::cursor::Goto(1, 1));
        // print!("{}", self.theme.outline.fg_string());
        print!("┌");
        Editor::draw_line();
        print!("┐");

        for x in 2..size.1 {
            print!(
                "{}│{}│",
                termion::cursor::Goto(1, x),
                termion::cursor::Goto(size.0, x)
            );
        }

        print!("└");
        Editor::draw_line();
        print!("┘");

        print!(
            // "{}├{}Text Buffer{}",
            "{}Text Buffer",
            termion::cursor::Goto(2, 1),
            // self.theme.foreground.fg_string(),
            // self.theme.outline.fg_string(),
        );

        print!(
            "{}├",
            termion::cursor::Goto(1, size.1 - self.terminal.height)
        );

        Editor::draw_line();
        print!("┤");

        print!(
            "{}Terminal",
            // "{}├{}Terminal",
            termion::cursor::Goto(2, size.1 - self.terminal.height),
            // self.theme.foreground.fg_string(),
        );
    }
    pub fn draw_line() {
        let size = terminal_size().unwrap();

        for _x in 1..size.0 - 1 {
            print!("─")
        }
    }
    pub fn clear() {
        // clear the terminal after
        print!(
            "{}{}{}{}{}",
            termion::cursor::Show,
            termion::clear::All,
            termion::cursor::SteadyBlock,
            termion::color::Reset.fg_str(),
            termion::color::Reset.bg_str(),
        );
    }
}
/// file based functions
impl Editor {
    pub fn save(&mut self) {
        // Multithread the save code

        // Take the current pathbuf and save all data there
    }
    pub fn set_path(&mut self) {}
    pub fn open(&mut self) {}
}
/// Util Implementation
impl Editor {
    pub fn size() -> (u16, u16) {
        let size = terminal_size().unwrap();
        size
    }
}
use crate::{terminal::Terminal, theme::Theme};
use serde_derive::Deserialize;
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::PathBuf,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, terminal_size};
