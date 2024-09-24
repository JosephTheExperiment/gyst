use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    main: LegOffSubcommands,
}

#[derive(Subcommand)]
enum LegOffSubcommands {
    /// Creates a new project
    New {},
    /// Install libraries from conan
    Install {},
    /// Add {.c, .cpp, .h, .hpp, section} to or from sections
    Add {},
    /// Delete {.c, .cpp, .h, .hpp, section} to or from sections
    Delete {},
    /// Set a variable to some value
    Set {},
    /// Add some optional features to the porject
    Init {},
    /// Build and run all sections and the main.{c, cpp} file
    Run {},
    /// Build all sections and the main.{c, cpp} if it exist
    Build {},
}