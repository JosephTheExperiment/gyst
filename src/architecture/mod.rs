mod macros;
mod utils;
use crate::printing::StylizedStrings;
use crate::{pub_struct, Cli};

pub enum CommandErrors {
    NameCorrection(ErrorMassage),
    InputPrompting(ErrorMassage),
    ValidateInput(ErrorMassage),
    ValidateState(ErrorMassage),
    Runtime(ErrorMassage),
    None,
}

pub_struct! {
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

pub_struct! {
    #[derive(Clone)]
    struct Header<T> {
        header: Option<String>,
        values: Vec<T>
    }
}

pub_struct! {
    struct CliData {
        description: StylizedStrings,
        command_data: Vec<CommandData>,
        read_more: Vec<StylizedStrings>
    }
}

pub_struct! {
    #[derive(Clone)]
    struct CommandData {
        name: String,
        description: StylizedStrings,
        detailed_description: StylizedStrings,
        examples: Vec<StylizedStrings>,
        arguments: Vec<Input>,
        options: Vec<Header<Input>>,
        read_more: Vec<StylizedStrings>,
        command_variants: Option<Vec<CommandVariant>>
    }
}

pub_struct! {
    #[derive(Clone)]
    struct CommandVariant {
        ty: CommandVariantTypes,
        data: CommandVariantData
    }
}

#[derive(Clone)]
pub enum CommandVariantTypes {
    Vcpkg,
    Conan,
    Hunter,
}

pub_struct! {
    #[derive(Clone)]
    struct CommandVariantData {
        examples: Option<Vec<StylizedStrings>>,
        arguments: Option<Vec<Input>>,
        options: Option<Vec<Header<Input>>>,
        read_more: Option<Vec<StylizedStrings>>
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
        possible_values: Option<Vec<String>>,
    },
    Arg {
        value: String,
        description: String,
        possible_values: Option<Vec<String>>,
    },
}
