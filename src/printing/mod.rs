mod help;
use std::io;
use crossterm::{execute, style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle}};

pub struct StylizedString(pub ContentStyle, pub String);
pub type StylizedStrings = Vec<StylizedString>;

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