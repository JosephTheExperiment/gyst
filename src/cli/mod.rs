mod macros;
mod styles;
use crate::{cli_build_interface, subcommands_build_interface};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

cli_build_interface!(Cli command(styles=styles::get_styles()) { subcommands GystSubcommand, });

subcommands_build_interface!(GystSubcommand {});