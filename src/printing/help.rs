use clap::builder::PossibleValue;

use super::{create_possible_values, HelpStyle, StylizedStrings, Verbosity};
use crate::architecture::{CommandData, Input};
use crate::cli_print;
use crate::printing::{stylized_print, stylized_prints, StylizedString};

impl CommandData {
    fn print_description(&self, verbosity: &Verbosity) -> std::io::Result<()> {
        match verbosity {
            Verbosity::Quite => {
                stylized_prints(self.description.clone())?;
            }
            _ => {
                stylized_prints(self.description.clone())?;
                cli_print!(empty line);
                stylized_prints(self.detailed_description.clone())?;
            }
        }

        Ok(())
    }

    fn print_usage(&self, style: &HelpStyle) -> std::io::Result<()> {
        cli_print!(header => "Usage", style.header);
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
        cli_print!(header => "Examples", style.header);
        for example in self.examples.clone() {
            cli_print!(subheader => style);
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

fn create_inputs_prints(inputs: Vec<Input>, style: &HelpStyle) -> Vec<StylizedStrings> {
    let mut inputs_print: Vec<StylizedStrings> = vec![];
    let mut max_first_half_length: usize = 0;

    for input in &inputs {
        let half_input_print = create_half_input_print(input.clone(), style);
        inputs_print.push(half_input_print.clone());

        if half_input_print.len() > max_first_half_length {
            max_first_half_length = half_input_print.len();
        }
    }

    for i in 0..inputs_print.len() {
        let mut white_spaces: String = "".to_string();

        for _ in 0..(max_first_half_length - inputs_print[i].len()) + 1 {
            white_spaces.push_str(" ");
        }

        inputs_print[i].push(StylizedString(style.default, white_spaces));

        match &inputs[i] {
            Input::Arg { description, .. } => {
                inputs_print[i].push(StylizedString(style.default, description.to_string()));
            }
            Input::Flag {
                description,
                default_value,
                ..
            } => {
                inputs_print[i].push(StylizedString(style.default, description.to_string()));
                if let Some(default_value) = default_value {
                    inputs_print[i].push(StylizedString(style.default, " ".to_string()));
                    inputs_print[i].push(StylizedString(
                        style.default,
                        format!("(default value: {})", default_value.to_string()),
                    ));
                }
            }
        }
    }

    return inputs_print;
}
