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
    /// Creates a new project or to an existing project
    New(NewArgs),
    /// Install libraries from conan
    Install(InstallArgs),
    /// Add {.c, .cpp, .h, .hpp, module} to or from other modules
    Add(AddArgs),
    /// Delete {.c, .cpp, .h, .hpp, module} to or from other modules
    Delete(DeleteArgs),
    /// Set a variable to some value
    Set {},
    /// Add some optional features to the porject
    Init {},
    /// Build and run all modules and the main.{c, cpp} file
    Run {},
    /// Build all modules and the main.{c, cpp} if it exist
    Build {},
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Lang {
    C,
    Cpp,
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
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
        /// File or module name
        #[arg(short, long)]
        name: String,

        /// module(path) for the file or module to be created to
        #[arg(long, default_value = ".")]
        to: PathBuf,

        /// Create a new file or module or replace the old one without asking
        #[arg(short, long)]
        force: bool,
    }
);

pub_struct!(
    #[derive(Args)]
    struct DeleteArgs {
        /// File or module name
        #[arg(short, long)]
        name: String,

        /// module(path) for the file or module to be deleted from
        #[arg(long, default_value = ".")]
        from: PathBuf,

        /// Delete a file or module without asking
        #[arg(short, long)]
        force: bool,
    }
);