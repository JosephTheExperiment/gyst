mod macros;
mod_all!(utils, cmake);
use crate::mod_all;
use crate::Cli;

pub enum SubcommandErrors {
    NameCorrection,
    InputPrompting,
    ValidateInput,
    ValidateState,
    Runtime,
    None,
}

pub trait Subcommand {
    fn name_correction(&self, cli: &Cli) -> Result<(), SubcommandErrors>;
    fn input_prompting(&self, cli: &Cli) -> Result<(), SubcommandErrors>;
    fn validate_input(&self, cli: &Cli) -> Result<(), SubcommandErrors>;
    fn validate_state(&self, cli: &Cli) -> Result<(), SubcommandErrors>;
    fn run(&self, cli: &Cli) -> Result<(), SubcommandErrors>;
    fn compilation(&self, cli: Cli, errors: SubcommandErrors) -> Result<(), SubcommandErrors>;
}