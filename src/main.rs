mod cli;
mod cmake;
mod gyst;
use clap::Parser;
use crate::cli::subcommands::Cli;

fn main() {
    let cli = Cli::parse();

    //You can check for the existence of subcommands, and if found use their
    //matches just as you would the top level cmd
    match &cli.command {
        _ => {}
    }
}
