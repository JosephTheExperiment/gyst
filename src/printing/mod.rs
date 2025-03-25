mod help;
use crate::pub_struct;
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::io;

#[derive(Clone)]
pub struct StylizedString(pub ContentStyle, pub String);
#[derive(Clone)]
pub struct StylizedStrings(pub Vec<StylizedString>);

impl StylizedString {
    fn len(&self) -> usize {
        let Self(_, string) = self;
        string.chars().count()
    }

    fn new() -> StylizedString {
        StylizedString(ContentStyle::new(), String::new())
    }
}

impl StylizedStrings {
    fn len(&self) -> usize {
        let Self(substrings) = self;
        let mut total_length: usize = 0;
        for substring in substrings {
            total_length += substring.len();
        }

        return total_length;
    }

    fn push(&mut self, string: StylizedString) {
        self.0.push(string);
    }

    fn new() -> StylizedStrings {
        StylizedStrings(vec![])
    }
}

impl IntoIterator for StylizedStrings {
    type Item = StylizedString;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub enum Verbosity {
    Quite,
    Default,
    Verbose,
    Debug,
}

pub_struct! {
    struct HelpStyle {
        header: ContentStyle,
        subheader: ContentStyle,
        contrast: ContentStyle,
        default: ContentStyle
    }
}

pub fn stylized_print(string: StylizedString) -> std::io::Result<()> {
    execute!(
        io::stdout(),
        SetStyle(string.0),
        Print(string.1),
        SetAttribute(Attribute::Reset),
        ResetColor,
    )?;

    Ok(())
}

pub fn stylized_prints(strings: StylizedStrings) -> std::io::Result<()> {
    for substring in strings {
        stylized_print(substring)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! header {
    ($header: literal, $style:expr) => {
        stylized_print(StylizedString($style, String::from($header) + ": "))?;
    };
}

fn new_subheader(white_spaces: u8) {
    print!("\n");
    for _ in 0..white_spaces {
        print!(" ")
    }
}

fn empty_line() {
    print!("\n\n");
}

fn create_possible_values(values: Vec<String>) -> String {
    let mut possible_values: String = String::from("<");
    for value in values {
        possible_values.push_str(&(value.to_uppercase() + "|"));
    }
    possible_values.pop();
    possible_values + ">"
}
