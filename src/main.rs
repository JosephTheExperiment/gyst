use clap::{Parser, Subcommand};
use args::Cli;
mod leg_off;
mod cmake;
mod args;

fn main() { 
    let cli = Cli::parse();
}