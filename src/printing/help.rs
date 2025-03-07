use crate::{
    architecture::{CommandData, Input},
    pub_struct,
};
use crate::printing::{StylizedString, stylized_print, stylized_prints};
use crossterm::style::ContentStyle;

pub_struct!{
    struct CommandHelpStyle {
        header: ContentStyle,
        subheader: ContentStyle,
        contrast: ContentStyle
    }
}

macro_rules! new_section {
    ($header:literal => $($($line:tt)*),*) => {
        
    };
}

pub fn command_help(command: CommandData, style: CommandHelpStyle) -> std::io::Result<()> {
    // Short description with additional details:
    stylized_prints(command.description)?;
    print!("\n\n");
    stylized_prints(command.detailed_description)?;

    print!("\n\n");

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

    print!("\n\n");

    // Examples:
    stylized_print(StylizedString(style.header, String::from("Examples:\n")))?;
    for example in command.examples {
        stylized_prints(example)?;
        print!("\n");
    } 

    print!("\n\n");

    

    Ok(())
}
