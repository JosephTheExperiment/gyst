mod macros;
use crate::cli_build_interface;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: GystSubcommand,
}

cli_build_interface!(GystSubcommand {});
