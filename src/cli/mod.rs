mod macros;
use crate::cli_interface;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: GystSubcommand,
}

cli_interface!(
    GystSubcommand {
        New {
            about => "Create new project or create to an existing project",
            long_about => "",
            flags {
                lang #[arg(long, value_enum)] : ProjectLang => "" 
            },
            enums {
                ProjectLang { C, Cpp }
            }
        }
    }
);