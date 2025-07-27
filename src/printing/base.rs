use crate::pub_struct;
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::io;

type TabSize = u8;
pub type WritingElements = Collection<WritingElement>;
pub type StylizedStrings = Collection<StylizedString>;

#[derive(Clone)]
pub struct Collection<T>(pub Vec<T>);

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Collection<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[derive(Clone)]
pub enum WritingElement {
    NewLine,
    Spaces(usize),
    Text(StylizedString),
    Paragraph(StylizedStrings),
    Header {
        header: StylizedString,
        elements: WritingElements,
    },
}

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
        default: ContentStyle,
        text: ContentStyle,
        tab: TabSize
    }
}

pub_struct! {
    #[derive(Clone)]
    struct StylizedString {
        style: ContentStyle,
        string: String
    }
}

impl StylizedString {
    pub fn len(&self) -> usize {
        self.string.chars().count()
    }

    pub fn push_str(&mut self, string: &str) {
        self.string.push_str(string);
    }

    pub fn new() -> StylizedString {
        StylizedString {
            style: ContentStyle::new(),
            string: String::new(),
        }
    }

    pub fn print(&self) -> Result<(), io::Error> {
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

impl StylizedStrings {
    pub fn len(&self) -> usize {
        let mut length: usize = 0;
        for substring in self {
            length += substring.len();
        }

        length
    }

    pub fn push(&mut self, string: StylizedString) {
        self.0.push(string);
    }

    pub fn new() -> StylizedStrings {
        Collection(vec![StylizedString::new()])
    }

    pub fn print(&self) -> Result<(), io::Error> {
        for substring in self {
            substring.print();
        }

        Ok(())
    }
}
