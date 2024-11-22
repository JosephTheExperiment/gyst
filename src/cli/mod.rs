mod macros;
use crate::{cli_build_interface, subcommands_build_interface};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: GystSubcommand,
}

subcommands_build_interface!(GystSubcommand {});