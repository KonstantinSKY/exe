#![allow(dead_code)]

use console::Style;


pub trait ApplyStyle {
    fn green(self) -> String;
    fn cyan(self) -> String;
    fn blue(self) -> String;
    fn red(self) -> String;
    fn yellow(self) -> String;
}

impl ApplyStyle for &str {
    fn green(self) -> String {
        Style::new().green().apply_to(self).to_string()
    }
    fn cyan(self) -> String {
        Style::new().cyan().apply_to(self).to_string()
    }
    fn blue(self) -> String {
        Style::new().blue().apply_to(self).to_string()
    }
    fn red(self) -> String {
        Style::new().red().apply_to(self).to_string()
    }
    fn yellow(self) -> String {
        Style::new().yellow().apply_to(self).to_string()
    }

}