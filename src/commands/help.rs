use crate::{architecture::{CommandData, StylizedStrings}, pub_struct};
use crossterm::{
    execute,
    style::{Attribute, ContentStyle, Print, ResetColor, SetAttribute, SetStyle},
};
use std::io;

pub_struct!(
    struct CommandHelpStyle {
        header: ContentStyle,
        list_points: ContentStyle,
        usage: ContentStyle,
        read_more: ContentStyle,
    }
);

pub fn stylized_printing(string: StylizedStrings) -> std::io::Result<()> {
    for substring in string {
        execute!(
            io::stdout(),
            SetStyle(substring.0),
            Print(substring.1),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?
    }

    Ok(())
}

pub fn command_help(command: CommandData, style: CommandHelpStyle) -> std::io::Result<()> {


    Ok(())
}
