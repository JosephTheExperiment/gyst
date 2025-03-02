mod macros;
mod_all!(utils, cmake);

use crate::{mod_all, pub_struct, Cli};

pub enum CommandErrors {
    NameCorrection,
    InputPrompting,
    ValidateInput,
    ValidateState,
    Runtime,
    None,
}

pub trait Subcommand {
    fn name_correction(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn input_prompting(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn validate_input(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn validate_state(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn run(&self, cli: &Cli) -> Result<(), CommandErrors>;
    fn compilation(&self, cli: Cli, errors: CommandErrors) -> Result<(), CommandErrors>;
}

pub_struct!(
    struct header<T> {
        header: String,
        values: Vec<T>,
    }
);

pub_struct!(
    struct CliData {
        description: String,
        command_data: Vec<CommandData>,
        read_more: String,
    }
);

pub_struct!(
    struct CommandData {
        name: String,
        description: String,
        detailed_description: String,
        examples: Vec<String>,
        required: Vec<Input>,
        options: Vec<header<Input>>,
        read_more: Vec<String>,
    }
);

pub_struct!(
    struct CommandDataOption {
        examples: Option<Vec<String>>,
        required: Option<Vec<Input>>,
        options: Option<Vec<header<Input>>>,
        read_more: Option<Vec<String>>,
    }
);

pub_struct!(
    struct CommandOptions {
        vcpkg: Option<CommandDataOption>,
        conan: Option<CommandDataOption>,
        hunter: Option<CommandDataOption>,
        github: Option<CommandDataOption>,
    }
);

pub enum Input {
    Flag {
        short: Option<String>,
        long: String,
        value: String,
        description: String,
        default_value: Option<String>,
        possible_values: Option<Vec<String>>,
    },
    Arg {
        value: String,
        description: String,
        default_value: Option<String>,
        possible_values: Vec<String>,
    },
}
