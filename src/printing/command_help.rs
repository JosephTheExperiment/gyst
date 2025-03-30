use super::{create_possible_values, HelpStyle, StylizedStrings, Verbosity};
use crate::architecture::{CommandData, Header, Input};
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
        cli_print!(header => "Usage", style);
        stylized_prints(StylizedStrings(vec![
            StylizedString(style.subheader, format!("gyst {}", self.name)),
            StylizedString::white_spaces(style, 1),
            StylizedString(style.contrast, "[OPTIONS]".to_string()),
            StylizedString::white_spaces(style, 1),
        ]))?;

        for arg in self.arguments.clone() {
            match arg {
                Input::Arg { value, .. } => {
                    stylized_print(StylizedString(style.contrast, value))?;
                    cli_print!(white space => style);
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn print_examples(&self, style: &HelpStyle) -> std::io::Result<()> {
        cli_print!(header => "Examples", style);
        for example in self.examples.clone() {
            cli_print!(subheader => style);
            stylized_prints(example)?;
        }

        Ok(())
    }

    fn print_header_input(header: Header<Input>, style: &HelpStyle) -> std::io::Result<()> {
        let inputs_prints: Vec<StylizedStrings> = create_inputs_prints(&header.values, style);

        if let Some(header_string) = header.header {
            cli_print!(header => header_string, style);
        }

        for input_print in inputs_prints {
            cli_print!(subheader => style);
            stylized_prints(input_print)?;
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
            input_print.push(StylizedString::white_spaces(style, 1));
            input_print.push(match possible_values {
                Some(x) => StylizedString(style.contrast, create_possible_values(x)),
                None => StylizedString(style.contrast, value),
            });
        }
    }

    return input_print;
}

fn create_inputs_prints(inputs: &Vec<Input>, style: &HelpStyle) -> Vec<StylizedStrings> {
    let mut inputs_prints: Vec<StylizedStrings> = vec![];
    let mut max_first_half_length: usize = 0;

    for input in inputs {
        let half_input_print = create_half_input_print(input.clone(), style);
        inputs_prints.push(half_input_print.clone());

        if half_input_print.len() > max_first_half_length {
            max_first_half_length = half_input_print.len();
        }
    }

    for i in 0..inputs_prints.len() {
        let length: usize = (max_first_half_length - inputs_prints[i].len()) + 1;
        inputs_prints[i].push(StylizedString::white_spaces(style, length));

        match &inputs[i] {
            Input::Arg { description, .. } => {
                inputs_prints[i].push(StylizedString(style.default, description.to_string()));
            }
            Input::Flag {
                description,
                default_value,
                ..
            } => {
                inputs_prints[i].push(StylizedString(style.default, description.to_string()));
                if let Some(default_value) = default_value {
                    inputs_prints[i].push(StylizedString::white_spaces(style, 1));
                    inputs_prints[i].push(StylizedString(
                        style.default,
                        format!("(default value: {})", default_value.to_string()),
                    ));
                }
            }
        }
    }

    return inputs_prints;
}
