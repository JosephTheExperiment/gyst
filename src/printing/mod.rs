mod base;
mod macros;
use base::{StylizedString, WritingElement, WritingElements};
use std::io;

impl WritingElements {
    fn print(&self) -> Result<(), io::Error> {
        for element in self {
            match element {
                WritingElement::NewLine => print!("\n"),
                WritingElement::Spaces(spaces_number) => {
                    let mut spaces: String = String::new();
                    for _ in 0..(*spaces_number as i32) {
                        spaces.push(' ');
                    }
                    print!("{}", spaces);
                }
                WritingElement::Text(string) => string.print()?,
                WritingElement::Paragraph(strings) => strings.print()?,
                WritingElement::Header { header, elements } => {
                    let mut header: StylizedString = header.clone();
                    header.push_str(":\n");
                    header.print()?;
                    Self::print(elements)?;
                }
            }
        }

        Ok(())
    }

    fn len(&self) -> usize {
        let mut length: usize = 0;
        for element in self {
            length += match element {
                WritingElement::NewLine => break,
                WritingElement::Spaces(spaces_number) => *spaces_number,
                WritingElement::Text(string) => string.len(),
                WritingElement::Paragraph(strings) => strings.len(),
                WritingElement::Header { .. } => break,
            }
        }

        length
    }
}
