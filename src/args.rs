use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    main: LegOffSubcommands,
}

#[derive(Subcommand)]
enum LegOffSubcommands {
    /// Creates a new project
    New(NewArgs),
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

#[derive(Args)]
struct NewArgs{
    /// Project's language
    #[arg(long, value_enum)]
    lang: Lang,
    
    /// Project's type
    #[arg(long, value_enum)]
    r#type: Type,
    
    /// Project's name
    #[arg(short, long)]
    name: String, 

    /// Build system generator for cmake
    #[arg(short = 'G', required = false)]
    g: String,
    
    /// Directory for the project
    #[arg(long, default_value = ".")]
    to: PathBuf,

    /// Unit testing framework
    #[arg(long, required = false)]
    test: bool,
    
    /// Add conanfile.py to be able to install libraries 
    #[arg(long, required = false)]
    conan: bool
}

#[derive(Clone, ValueEnum)]
enum Lang {
    C,
    Cpp,
}

#[derive(Clone, ValueEnum)]
enum Type {
    App,
    SharedLib,
    StaticLib,
    IncludeLib,
    ModuleLib,
}
