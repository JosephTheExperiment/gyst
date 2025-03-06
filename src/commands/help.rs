use crate::{
    architecture::{CommandData, Input, StylizedString, StylizedStrings},
    pub_struct,
};
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::io;

pub_struct!(
    struct CommandHelpStyle {
        header: ContentStyle,
        subheader: ContentStyle,
        contrast: ContentStyle,
    }
);

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

pub fn command_help(command: CommandData, style: CommandHelpStyle) -> std::io::Result<()> {
    // Short description with additional details:
    stylized_prints(command.description)?;
    println!("");
    stylized_prints(command.detailed_description)?;

    // Usage:
    stylized_prints(vec![
        StylizedString(style.header, String::from("Usage: ")),
        StylizedString(style.subheader, format!("gyst {} ", command.name)),
        StylizedString(style.contrast, String::from("[OPTIONS] ")),
    ])?;

    for arg in command.arguments {
        match arg {
            Input::Arg { value, .. } => {
                stylized_print(StylizedString(style.contrast, value + " "))?
            }
            _ => (),
        }
    }

    // Examples:
    

    Ok(())
}
