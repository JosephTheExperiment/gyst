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
    Install(InstallArgs),
    /// Add {.c, .cpp, .h, .hpp, section} to or from sections
    Add(AddArgs),
    /// Delete {.c, .cpp, .h, .hpp, section} to or from sections
    Delete(DeleteArgs),
    /// Set a variable to some value
    Set {},
    /// Add some optional features to the porject
    Init {},
    /// Build and run all sections and the main.{c, cpp} file
    Run {},
    /// Build all sections and the main.{c, cpp} if it exist
    Build {},
}

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

pub_struct!(
    #[derive(Args)]
    struct NewArgs {
        /// Project language
        #[arg(long, value_enum)]
        lang: Lang,

        /// Project type
        #[arg(long, value_enum)]
        r#type: Type,

        /// Project name
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

pub_struct!(
    #[derive(Args)]
    struct InstallArgs {
        /// Library name
        #[arg(short, long)]
        name: String,

        /// Library version
        #[arg(short, long)]
        version: String,
    }
);

pub_struct!(
    #[derive(Args)]
    struct AddArgs {
        /// File or section name
        #[arg(short, long)]
        name: String,

        /// Section(path) for the file or section to be created to
        #[arg(long, default_value = ".")]
        to: PathBuf,

        /// Create a new file or section or replace the old one without asking
        #[arg(short, long)]
        force: bool,
    }
);

pub_struct!(
    #[derive(Args)]
    struct DeleteArgs {
        /// File or section name
        #[arg(short, long)]
        name: String,

        /// Section(path) for the file or section to be deleted from
        #[arg(long, default_value = ".")]
        from: PathBuf,

        /// Delete a file or section without asking
        #[arg(short, long)]
        force: bool,
    }
);