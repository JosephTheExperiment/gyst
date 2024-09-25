use clap::Parser;
use args::Cli;
mod leg_off;
mod cmake;
mod args;

fn main() { 
    let _cli = Cli::parse();
}