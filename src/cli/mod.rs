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

#[derive(Subcommand)]
pub enum GystSubcommand {
    #[command(about = "", long_about = "")]
    New {}
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Lang {
    C,
    Cpp,
}
