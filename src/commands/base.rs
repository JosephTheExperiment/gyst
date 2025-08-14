use crate::printing::base::StylizedStrings;
use crate::pub_struct;

pub enum CliErrorsTypes {
    RequiredInputPrompting(CliErrorMassage),
    CommandNameCorrection(CliErrorMassage),
    ValidateInput(CliErrorMassage),
    ValidateState(CliErrorMassage),
    Runtime(CliErrorMassage),
    None,
}

pub_struct! {
    struct CliErrorMassage {}
}

// In progress :)
// pub trait Subcommand {
//     fn required_input_prompting(&self, cli: &Cli);
//     fn command_name_correction(&self, cli: &Cli);
//     fn validate_input(&self, cli: Cli) -> Result<Cli, CommandErrors>;
//     fn validate_state(&self, cli: Cli) -> Result<Cli, CommandErrors>;
//     fn run(&self, cli: Cli) -> Result<Cli, CommandErrors>;
//     fn compilation(&self, cli: Cli, errors: CommandErrors) -> Result<(), CommandErrors>;
// }

pub_struct! {
    #[derive(Clone)]
    struct CliData {
        name: String,
        verison: String,
        description: String,
        command_data: Vec<CommandData>,
        read_more: Vec<ReadMore>
    }
}

pub_struct! {
    #[derive(Clone)]
    struct ReadMore {}
}

pub_struct! {
    #[derive(Clone)]
    struct CommandData {
        name: String,
        description: String,
        detailed_description: String,
        examples: Vec<Example>,
        arguments: Vec<Input>,
        flags: Vec<Header<Input>>,
        read_more: Vec<StylizedStrings>
    }
}

pub_struct! {
    #[derive(Clone)]
    struct Header<T> {
        header: Option<String>,
        values: Vec<T>
    }
}

pub_struct! {
    #[derive(Clone)]
    struct Example {}
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
