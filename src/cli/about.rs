use crate::pub_struct;
use std::collections::HashMap;

macro_rules! BulidSubcommandAbout {
    ($name:ident about: $about:expr, long_about: $long_about:expr, $($flag:ident: $flag_about:expr)+) => {
        fn $name() -> SubcommandAbout {
            
        } 
    };
}

pub_struct!(
    struct SubcommandAbout {
        about: String,
        long_about: String,
        flags: HashMap<String, String>,
    }
);
