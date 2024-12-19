mod_all!(new);
use crate::{cli, mod_all, cli::styles::get_styles};
use crate::cli::subcommands::new::NewArgs;
use clap::{Parser, Subcommand};
use paste::paste;

cli!(
    #[command(styles=get_styles())]
    Cli {
        Commands { New };
    }
);
