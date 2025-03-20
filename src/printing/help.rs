use inquire::length;

use super::{create_possible_values, HelpStyle, StylizedStrings, Verbosity};
use crate::architecture::{CommandData, Input};
use crate::header;
use crate::printing::{empty_line, new_subheader, stylized_print, stylized_prints, StylizedString};

fn print_description(
    description: StylizedStrings,
    detailed_description: StylizedStrings,
    verbosity: &Verbosity,
) -> std::io::Result<()> {
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

fn print_usage(
    command_name: String,
    arguments: Vec<Input>,
    style: &HelpStyle,
) -> std::io::Result<()> {
    header!("Usage", style.header);
    stylized_prints(vec![
        StylizedString(style.subheader, format!("gyst {} ", command_name)),
        StylizedString(style.contrast, "[OPTIONS] ".to_string()),
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

fn create_half_input_print(input: Input, style: &HelpStyle) -> StylizedStrings {
    let mut half_input_print: StylizedStrings = vec![];

    match input {
        Input::Arg {
            value,
            possible_values,
            ..
        } => match possible_values {
            Some(values) => half_input_print.push(StylizedString(
                style.contrast,
                create_possible_values(values) + " ",
            )),
            None => half_input_print.push(StylizedString(style.contrast, value + " ")),
        },
        Input::Flag {
            short,
            long,
            value,
            possible_values,
            ..
        } => { 
            match short {
                Some(short) => {
                    half_input_print.push(StylizedString(style.subheader, short));
                    half_input_print.push(StylizedString(style.default, ", ".to_string()));
                    half_input_print.push(StylizedString(style.subheader, long + " "));
                }
                None => {
                    half_input_print.push(StylizedString(style.default, "    ".to_string()));
                    half_input_print.push(StylizedString(style.subheader, long + " "));
                }
            }
            match possible_values {
                Some(values) => {
                    
                }
                None => {
                }
            }
        }
    }

    half_input_print
}

fn print_input(inputs: Vec<Input>, style: &HelpStyle) -> std::io::Result<()> {
    let mut inputs_prints: Vec<StylizedStrings> = vec![];
    let mut max_length: usize = 0;

    for input in inputs {   
        inputs_prints.push(create_half_input_print(input, style));
        

    }

    Ok(())
}

pub fn command_help(
    command: CommandData,
    style: HelpStyle,
    verbosity: Verbosity,
) -> std::io::Result<()> {
    print_description(
        command.description,
        command.detailed_description,
        &verbosity,
    )?;
    empty_line();

    print_usage(command.name, command.arguments, &style)?;
    empty_line();

    print_example(command.examples, &style)?;
    empty_line();

    // Arguments:
    header!("Arguments", style.header);

    Ok(())
}
