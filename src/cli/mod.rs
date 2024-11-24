mod macros;
use crate::{cli_build_interface, subcommands_build_interface};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

cli_build_interface!(Cli { subcommands GystSubcommand, });

subcommands_build_interface!(GystSubcommand {});