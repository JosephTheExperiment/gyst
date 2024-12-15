mod macros; mod styles; mod architecture;
use crate::{subcommand, cli};
use crate::cli::styles::get_styles;
use clap::{Args, Parser, Subcommand};
use paste::paste;

subcommand!(
    New {
        about => "hi";
        long_about => "haaaay";
    }
);

cli!(
    #[command(styles=get_styles())] 
    Cli {
        Commands { New };
    }
);