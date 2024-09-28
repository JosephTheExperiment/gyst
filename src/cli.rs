use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

macro_rules! pub_struct {
    ($(#[$struct_attr:meta])* struct $name:ident {$($(#[$field_attr:meta])* $field:ident: $t:ty,)*}) => {
        $(#[$struct_attr])*
        pub struct $name {
            $($(#[$field_attr])*
            pub $field: $t,)*
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: LegOffSubcommands,
}

#[derive(Subcommand)]
pub enum LegOffSubcommands {
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

pub_struct!(
    #[derive(Args)]
    struct NewArgs {
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
        #[arg(short = 'G')]
        g: Option<String>,

        /// Directory for the project
        #[arg(long, default_value = ".")]
        to: PathBuf,

        /// Unit testing framework
        #[arg(long)]
        test: bool,

        /// Add conanfile.py to be able to install libraries
        #[arg(long)]
        conan: bool,
    }
);

#[derive(Clone, ValueEnum)]
pub enum Lang {
    C,
    Cpp,
}

#[derive(Clone, ValueEnum)]
pub enum Type {
    App,
    SharedLib,
    StaticLib,
    IncludeLib,
    ModuleLib,
}
