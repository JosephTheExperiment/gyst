use crate::printing::{stylized_print, stylized_prints, StylizedString};
use crate::{
    architecture::{CommandData, Input},
    pub_struct,
};
use crossterm::style::ContentStyle;

pub_struct! {
    struct CommandHelpStyle {
        header: ContentStyle,
        subheader: ContentStyle,
        contrast: ContentStyle
    }
}

macro_rules! header {
    ($header: literal, $style:expr) => {
        stylized_print(StylizedString($style, String::from($header) + ": "))?;
    };
}

fn new_subheader(white_spaces: u8) {
    print!("\n");
    for _ in 0..white_spaces {
        print!(" ")
    }
}

fn empty_line() {
    print!("\n\n");
}

pub fn command_help(command: CommandData, style: CommandHelpStyle) -> std::io::Result<()> {
    // Short description with additional details:
    stylized_prints(command.description)?;
    empty_line();
    stylized_prints(command.detailed_description)?;
    empty_line();

    // Usage:
    header!("Usage", style.header);
    stylized_prints(vec![
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
    empty_line();

    // Examples:
    header!("Examples", style.header);
    for example in command.examples {
        new_subheader(2);
        stylized_prints(example)?;
    }
    empty_line();

    

    Ok(())
}
