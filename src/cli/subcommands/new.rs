use crate::subcommand;
use clap::{Args, ValueEnum};
use paste::paste;

subcommand!(
    New {
        about => "Creates new project or adds a target to an existing project";
        long_about => "who knows?";
        args {
            #[arg(short, long)] name: String => "Project name" 
            => "NOTE : This flag isn't required if you are adding to an existing project",
            #[arg(long)] lang: ProjectLang => "Project name"
        };
        options {
            #[arg(long, value_enum, default_value_t = ProjectType::App)] r#type: ProjectType => "Project type",
            #[arg(long, default_value_t = String::from("./"))] to: String => "Specifies a directory for the project" 
        };
        enums {
            ProjectLang { C => "C project", Cpp => "C++ project" },
            ProjectType { 
                App => "Apps have a main file and from it the app starts running", 
                SharedLib => "Shared or dynamic librarys are compiled and used as a separate file at runtime",
                StaticLib => "Static librarys are compiled separately or with some app and used as one with the app at runtime", 
                IncludeLib => "Include librarys are made of include files (e.g., .h, .hpp) and compiled with the app at runtime" 
            }
        };
    }
);