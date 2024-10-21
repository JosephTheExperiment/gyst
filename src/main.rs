mod cli;
mod cmake;
mod gyst;
use clap::Parser;
use cli::{Cli, LegOffSubcommands};

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    //matches just as you would the top level cmd
    match &cli.command {
        LegOffSubcommands::New(args) => {
            println!("New name {}", args.name);
        }
        LegOffSubcommands::Install(args) => {
            println!("Install name {}", args.name);
        }
        _ => {}
    }
}
