mod base;
mod macros;
use base::{ElementList, WritingElement};
use std::io;

fn writing_device(writing_elemetnts: ElementList) -> Result<(), io::Error> {
    for element in &writing_elemetnts {
        match element {
            WritingElement::Tab(tab_size) => {
                let mut tab: String = String::new();
                for _ in 0..(*tab_size as i8) {
                    tab.push(' ');
                }
                print!("{}", tab);
            }
            WritingElement::Spaces(spaces_number) => {
                let mut spaces: String = String::new();
                for _ in 0..(*spaces_number as i32) {
                    spaces.push(' ');
                }
                print!("{}", spaces);
            }
            WritingElement::Text(string) => string.print()?,
            WritingElement::Paragraph(strings) => strings.print()?,
            //WritingElement::Header(list) => writing_device(list.clone())?,
            _ => {}
        }
    }

    Ok(())
}
