use crate::subcommand;
use clap::{Args, ValueEnum};
use paste::paste;

subcommand!(
    New {
        about => "Creates new project or adds a target to an existing project";
        long_about => "who knows?";
        args {
            [short, long] name: String => "Project name" 
            => "NOTE : This flag isn't required if you are adding to an existing project",
            [long] lang: ProjectLang => "Project name"
        };
        options {
            [long, value_enum] r#type: ProjectType = ProjectType::App => "Project type",
            [long, value_name = "path"] to: String = String::from("./") => "Specifies a directory for the project" 
        };
        enums {
            ProjectLang { C => "C project", Cpp => "C++ project" },
            ProjectType { 
                App => "Apps have a main file and from it the app starts running", 
                SharedLib => "Shared or dynamic librarys are compiled and used as a separate file at runtime",
                StaticLib => "Static librarys are compiled separately or with some app and to be used it needs to be compiled with the app", 
                IncludeLib => "Include librarys are made of include files (e.g., .h, .hpp) and to be used it needs to be compiled with the app" 
            }
        };
    }
);