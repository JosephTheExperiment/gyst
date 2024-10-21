use std::collections::HashMap;

macro_rules! BuildAbout {
    ($subcommand:ident about: $about:expr, long_about: $long_about:expr, flags {$($flag:ident: $flag_about:expr),+}) => {
        pub fn $subcommand(about_type: AboutType) -> String {
            let mut flags: HashMap<String, String> = HashMap::from([$(("$flag".to_string(), $flag_about.to_string())),+]);
            match about_type {
                AboutType::About => $about.to_string(),
                AboutType::LongAbout => $long_about.to_string(),
                AboutType::Flag { flag } => flags.remove(&flag).expect("The flag dose not exist"),
            }
        }
    };
}

pub enum AboutType {
    About,
    LongAbout,
    Flag { flag: String },
}

BuildAbout!(new_about
about: "Create new project or create to an existing project",
long_about: "",
flags {
    lang: "Project language",
    r#type: "Project type",
    name: "Project name",
    G: "Build system generator for cmake",
    to: "Directory for the project",
    test: "Specify a unit testing framework add tests to your project and enable testing",
    conan: "Add conanfile.py to be able to install libraries via conan"
});
