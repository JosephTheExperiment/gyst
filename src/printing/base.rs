use crate::pub_struct;
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::{io, sync::Arc};

type TabSize = u8;

#[derive(Clone)]
pub struct ElementList(pub Arc<[WritingElement]>);

impl<'a> IntoIterator for &'a ElementList {
    type Item = &'a WritingElement;
    type IntoIter = std::slice::Iter<'a, WritingElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub enum WritingElement {
    Tab(TabSize),
    Spaces(usize),
    Text(StylizedString),
    Paragraph(StylizedStrings),
    Lines(Arc<[ElementList]>),
    Header(ElementList),
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

#[derive(Clone)]
pub struct StylizedStrings(pub Vec<StylizedString>);

impl IntoIterator for StylizedStrings {
    type Item = StylizedString;
    type IntoIter = std::vec::IntoIter<StylizedString>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a StylizedStrings {
    type Item = &'a StylizedString;
    type IntoIter = std::slice::Iter<'a, StylizedString>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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
        StylizedStrings(vec![StylizedString::new()])
    }

    pub fn print(&self) -> Result<(), io::Error> {
        for substring in self {
            substring.print();
        }

        Ok(())
    }
}
