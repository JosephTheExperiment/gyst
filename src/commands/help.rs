use crate::{
    architecture::{CommandData, StylizedString},
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
        list_points: ContentStyle,
        usage: ContentStyle,
        read_more: ContentStyle,
    }
);

pub fn stylized_printing(string: StylizedString) -> std::io::Result<()> {
    execute!(
        io::stdout(),
        SetStyle(string.0),
        Print(string.1),
        SetAttribute(Attribute::Reset),
        ResetColor
    )?;

    Ok(())
}

pub fn command_help(command: CommandData, style: CommandHelpStyle) -> std::io::Result<()> {
    Ok(())
}
