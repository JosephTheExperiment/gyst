use crate::pub_struct;
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::io;

pub enum Verbosity {
    Quite,
    Default,
    Verbose,
    Debug,
}

pub_struct! {
    struct CliStyle {
        header: ContentStyle,
        subheader: ContentStyle,
        contrast: ContentStyle,
        text: ContentStyle,
        tab: String
    }
}

pub_struct! {
    #[derive(Clone)]
    struct StylizedString {
        style: ContentStyle,
        string: String
    }
}

pub_struct! {
    #[derive(Clone)]
    struct StylizedStrings{ substrings: Vec<StylizedString> }
}

impl StylizedString {
    fn len(&self) -> usize {
        self.string.chars().count()
    }

    fn push_str(&mut self, string: &str) {
        self.string.push_str(string);
    }

    fn new() -> StylizedString {
        StylizedString {
            style: ContentStyle::new(),
            string: String::new(),
        }
    }

    fn print(&mut self) -> std::io::Result<()> {
        execute!(
            io::stdout(),
            SetStyle(self.style),
            Print(self.string.clone()),
            SetAttribute(Attribute::Reset),
            ResetColor,
        )?;

        Ok(())
    }
}

impl IntoIterator for StylizedStrings {
    type Item = StylizedString;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.substrings.into_iter()
    }
}

impl StylizedStrings {
    fn len(&mut self) -> usize {
        let mut length: usize = 0;
        for substring in self.clone() {
            length += substring.len();
        }

        length
    }

    fn push(&mut self, string: StylizedString) {
        self.substrings.push(string);
    }

    fn new() -> StylizedStrings {
        StylizedStrings {
            substrings: vec![StylizedString::new()],
        }
    }

    fn stylized_prints(&mut self) -> std::io::Result<()> {
        for mut substring in self.clone() {
            substring.print();
        }

        Ok(())
    }
}
