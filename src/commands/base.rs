use crate::printing::StylizedStrings;
use crate::pub_struct;

pub enum CommandErrors {
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

// pub trait Command {
//     fn validate_input(&self, cli: &Cli) -> Result<(), CommandErrors>;
//     fn validate_state(&self, cli: &Cli) -> Result<(), CommandErrors>;
//     fn run(&self, cli: &Cli) -> Result<(), CommandErrors>;
//     fn compilation(&self, cli: Cli, errors: CommandErrors) -> Result<(), CommandErrors>;
// }

pub_struct! {
    #[derive(Clone)]
    struct Header<T> {
        header: Option<String>,
        values: Vec<T>
    }
}

pub_struct! {
    struct CliData {
        name: String,
        verison: String,
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
        read_more: Vec<StylizedStrings>
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
