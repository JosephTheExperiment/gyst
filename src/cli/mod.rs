mod cli_build;
mod styles;
use crate::{subcommand, subcommands_enum, cli};
use crate::cli::styles::get_styles;
use clap::{Args, Parser, Subcommand};
use paste::paste;

subcommand!(
    #[] New {
        about => "hi",
        long_about => "haaaay"
    }
);

subcommands_enum!(
    #[] Commands {
        #[] New
    }
);

cli!(
    #[command(styles=get_styles())] Cli {
        subcommands : Commands
    }
);