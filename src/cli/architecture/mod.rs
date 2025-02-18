mod macros;
mod_all!(utils, cmake);
use std::io::Stderr;

use crate::{Cli, mod_all};

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

struct CliData {
    description: String,
    command_data: CommandData
}

struct CommandData {
    description: String,
    detailed_description: String,
    examples: Vec<String>,
    required: Vec<Input>,
    options: Vec<(String, Vec<Input>)>,
    read_more: String
}

pub enum Input {
    Flag {
        short: String,
        long: String,
        value: String,
        description: String,
    },
    Arg {
        value: String,
        description: String,
    }
}