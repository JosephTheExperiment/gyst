use crate::subcommand;
use clap::Args;
use paste::paste;

subcommand!(
    New {
        about => "hi";
        long_about => "haaaay";
    }
);