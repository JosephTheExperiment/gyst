use crate::{
    cli::architecture::{CommandData, CommandOptions},
    pub_struct,
};
use std::io::{stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub_struct!(
    struct HelpStyle {
        
    }
);

macro_rules! combine_command_data {
    ( $($options:ident),* ) => {
        fn combine_command_data(command: CommandData, options: CommandOptions) -> CommandData {
            let mut command: CommandData = command;

            $(
                if let Some(x) = options.$options {
                    if let Some(x) = x.examples { command.examples.extend(x) }
                    if let Some(x) = x.required { command.required = x }
                    if let Some(x) = x.options { command.options = x }
                    if let Some(x) = x.read_more { command.read_more.extend(x) }
                }
            )*

            return command;
        }
    };
}

combine_command_data!(vcpkg, conan, hunter, github);

pub fn command_help(command: CommandData, style: HelpStyle) -> std::io::Result<()> {
    

    Ok(())
}
