use crate::printing::{empty_line, new_subheader, stylized_print, stylized_prints, StylizedString};
use crate::{header, SS};
use crate::{
    architecture::{CommandData, Input},
    pub_struct,
};
use crossterm::style::ContentStyle;
use super::{HelpStyle, StylizedStrings, Verbosity};

fn print_description(description: StylizedStrings, detailed_description: StylizedStrings, verbosity: &Verbosity) -> std::io::Result<()> {
    match verbosity {
        Verbosity::Quite => {
            stylized_prints(description)?;
        }
        _ => {
            stylized_prints(description)?;
            empty_line();
            stylized_prints(detailed_description)?;
        }
    }

    Ok(())
}

fn print_usage(command_name: String, arguments: Vec<Input>, style: &HelpStyle) -> std::io::Result<()> {
    header!("Usage", style.header);
    stylized_prints(vec![
        StylizedString(style.subheader, format!("gyst {} ", command_name)),
        StylizedString(style.contrast, String::from("[OPTIONS] ")),
    ])?;

    for arg in arguments {
        match arg {
            Input::Arg { value, .. } => {
                stylized_print(StylizedString(style.contrast, value + " "))?
            }
            _ => (),
        }
    }

    Ok(())
}

fn print_example(examples: Vec<StylizedStrings>, style: &HelpStyle) -> std::io::Result<()> {
    header!("Examples", style.header);
    for example in examples {
        new_subheader(2);
        stylized_prints(example)?;
    }

    Ok(())
}

fn print_input(inputs: Vec<Input>, style: &HelpStyle) -> std::io::Result<()> {
    let mut inputs_prints: Vec<StylizedStrings> = vec![];

    for input in inputs {
        let mut pushed_value: StylizedStrings = vec![];
        
        match input {
            Input::Arg {
                value,
                possible_values,
                ..
            } => {
                
                match possible_values {
                    Some(values) => {
                        
                    }
                    None => { pushed_value.push(StylizedString(style.subheader, value + " ")) }
                }

            }
            Input::Flag {
                short,
                long,
                value,
                description,
                default_value,
                possible_values,
            } => {}
        }
    }

    Ok(())
}

pub fn command_help(command: CommandData, style: HelpStyle, verbosity: Verbosity) -> std::io::Result<()> {
    print_description(command.description, command.detailed_description, &verbosity)?;
    empty_line();

    print_usage(command.name, command.arguments, &style)?;
    empty_line();

    print_example(command.examples, &style)?;
    empty_line();

    // Arguments:
    header!("Arguments", style.header);

    Ok(())
}
