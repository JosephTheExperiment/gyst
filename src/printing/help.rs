use super::{HelpStyle, StylizedStrings, Verbosity};
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

// fn create_half_input_print(input: Input, style: &HelpStyle) -> StylizedStrings {

// }

// fn print_input(inputs: Vec<Input>, style: &HelpStyle) -> std::io::Result<()> {
//     let mut inputs_prints: Vec<StylizedStrings> = vec![];
//     let mut max_length: usize = 0;

//     for input in inputs {
//         inputs_prints.push(create_half_input_print(input, style));
//     }

//     Ok(())
// }

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
