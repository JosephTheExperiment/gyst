use clap::builder::PossibleValue;

use super::{create_possible_values, HelpStyle, StylizedStrings, Verbosity};
use crate::architecture::{CommandData, Input};
use crate::header;
use crate::printing::{empty_line, new_subheader, stylized_print, stylized_prints, StylizedString};

impl CommandData {
    fn print_description(&self, verbosity: &Verbosity) -> std::io::Result<()> {
        match verbosity {
            Verbosity::Quite => {
                stylized_prints(self.description.clone())?;
            }
            _ => {
                stylized_prints(self.description.clone())?;
                empty_line();
                stylized_prints(self.detailed_description.clone())?;
            }
        }

        Ok(())
    }

    fn print_usage(&self, style: &HelpStyle) -> std::io::Result<()> {
        header!("Usage", style.header);
        stylized_prints(StylizedStrings(vec![
            StylizedString(style.subheader, format!("gyst {} ", self.name)),
            StylizedString(style.contrast, "[OPTIONS] ".to_string()),
        ]))?;

        for arg in self.arguments.clone() {
            match arg {
                Input::Arg { value, .. } => {
                    stylized_print(StylizedString(style.contrast, value + " "))?
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn print_examples(&self, style: &HelpStyle) -> std::io::Result<()> {
        header!("Examples", style.header);
        for example in self.examples.clone() {
            new_subheader(2);
            stylized_prints(example)?;
        }

        Ok(())
    }
}

fn create_half_input_print(input: Input, style: &HelpStyle) -> StylizedStrings {
    let mut input_print: StylizedStrings = StylizedStrings::new();

    match input {
        Input::Arg {
            value,
            possible_values,
            ..
        } => input_print.push(match possible_values {
            Some(x) => StylizedString(style.contrast, create_possible_values(x)),
            None => StylizedString(style.contrast, value),
        }),
        Input::Flag {
            short,
            long,
            value,
            possible_values,
            ..
        } => {
            input_print.push(match short {
                Some(x) => StylizedString(style.subheader, format!("-{x}")),
                None => StylizedString(style.default, "    ".to_string()),
            });
            input_print.push(StylizedString(style.default, " ,".to_string()));
            input_print.push(StylizedString(style.subheader, format!("--{long}")));
            input_print.push(StylizedString(style.default, " ".to_string()));
            input_print.push(match possible_values {
                Some(x) => StylizedString(style.contrast, create_possible_values(x)),
                None => StylizedString(style.contrast, value),
            });
        }
    }

    return input_print;
}

fn create_inputs_print(inputs: Vec<Input>, style: &HelpStyle) -> Vec<StylizedStrings> {
    let mut inputs_print: Vec<StylizedStrings> = vec![];
    let mut max_first_half_length: usize = 0;

    for input in inputs {
        let half_input_print = create_half_input_print(input, style);
        inputs_print.push(half_input_print.clone());

        if half_input_print.len() > max_first_half_length {
            max_first_half_length = half_input_print.len();
        }
    }

    return inputs_print;
}

pub fn command_help(
    command: CommandData,
    style: HelpStyle,
    verbosity: Verbosity,
) -> std::io::Result<()> {
    command.print_description(&verbosity)?;
    empty_line();

    command.print_usage(&style)?;
    empty_line();

    command.print_examples(&style)?;
    empty_line();

    Ok(())
}
