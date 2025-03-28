mod macros;
mod_all!(utils, cmake);
use crate::{mod_all, pub_struct, Cli};
use crate::printing::StylizedStrings;

pub enum CommandErrors {
    NameCorrection(ErrorMassage),
    InputPrompting(ErrorMassage),
    ValidateInput(ErrorMassage),
    ValidateState(ErrorMassage),
    Runtime(ErrorMassage),
    None,
}

pub_struct!{
    struct ErrorMassage {
        error: String,
        massage: String,
        suggestions: Vec<String>
    }
}

pub trait Command {
    fn name_correction(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn input_prompting(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn validate_input(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn validate_state(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn run(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn compilation(&self, cli: Cli, errors: CommandErrors) -> Result<(), CommandErrors>;
}

pub_struct!{
    struct Header<T> {
        header: Option<String>,
        values: Vec<T>
    }
}

pub_struct!{
    struct CliData {
        description: StylizedStrings,
        command_data: Vec<CommandData>,
        read_more: Vec<StylizedStrings>
    }
}

pub_struct!{
    struct CommandData {
        name: String,
        description: StylizedStrings,
        detailed_description: StylizedStrings,
        examples: Vec<StylizedStrings>,
        arguments: Vec<Input>,
        options: Vec<Header<Input>>,
        read_more: Vec<StylizedStrings>,
        data_options: Option<CommandOptions>
    }
}

pub_struct!{
    struct CommandDataOptions {
        examples: Option<Vec<StylizedStrings>>,
        required: Option<Vec<Input>>,
        options: Option<Vec<Header<Input>>>,
        read_more: Option<Vec<StylizedStrings>>
    }
}

pub_struct!{
    struct CommandOptions {
        vcpkg: Option<CommandDataOptions>,
        conan: Option<CommandDataOptions>,
        hunter: Option<CommandDataOptions>,
        github: Option<CommandDataOptions>
    }
}

#[derive(Clone)]
pub enum Input {
    Flag {
        short: Option<char>,
        long: String,
        value: String,
        description: String,
        default_value: Option<String>,
        possible_values: Option<Vec<String>>
    },
    Arg {
        value: String,
        description: String,
        possible_values: Option<Vec<String>>
    },
}
