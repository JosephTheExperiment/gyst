use crate::pub_struct;
use std::collections::HashMap;

macro_rules! BulidSubcommandAbout {
    ($subcommand:ident about: $about:expr, long_about: $long_about:expr, flags: {$($flag:ident: $flag_about:expr),+}) => {
        fn $subcommand() -> SubcommandAbout {
            let mut flags: HashMap<String, String> = HashMap::new();
            $(flags.insert("$flag".to_string(),$flag_about.to_string());)+
            return SubcommandAbout {
                about: $about.to_string(),
                long_about: $long_about.to_string(),
                flags: flags
            };
        }
    };
}

macro_rules! GetSubcommandAbout {
    ($subcommand:ident => about: $about:ident) => {

    };
}

pub_struct!(
    struct SubcommandAbout {
        about: String,
        long_about: String,
        flags: HashMap<String, String>,
    }
);

BulidSubcommandAbout!(new_about
about: "Create new project or create to an existing project",
long_about: "",
flags: {
    lang: "Project language",
    r#type: "Project type",
    name: "Project name",
    G: "Build system generator for cmake",
    to: "Directory for the project",
    test: "Specify a unit testing framework add tests to your project and enable testing",
    conan: "Add conanfile.py to be able to install libraries via conan"
});
