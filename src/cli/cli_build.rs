pub struct SubcommandInfo {
    pub name: String,
    pub about: String,
    pub long_about: String,
    pub args: Vec<String>,
    pub options: Vec<String>
}

#[macro_export]
macro_rules! subcommand_args {
    (
        $($subcommand_args_attr:meta),* $subcommand_args:ident {
            name => $subcommand:ident,
            about => $about:expr,
            long_about => $long_about:expr,
            fn_info => $fn_info:ident
            $(,
                args {
                    $($($arg_attr:meta),* $arg:ident : $arg_type:ty => $arg_des:expr),*
                }
            )?
            $(,
                options {
                    $($($option_attr:meta),* $option:ident : $option_type:ty => $option_des:expr),*
                }
            )?
            $(,
                enums {
                    $($($enum_attr:meta),* $enum_name:ident { $($enum:ident => $enum_des:expr),* }),*
                }
            )?
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                pub enum $enum_name {
                    $(
                        #[doc = $enum_des]
                        $enum
                    ),*
                }
            )*
        )?

        pub fn $fn_info() -> SubcommandInfo {
            SubcommandInfo {
                name: String::from(stringify!($subcommand)),
                about: String::from($about),
                long_about: String::from($long_about),
                args: vec![$($(String::from(stringify!($arg))),*)?],
                options: vec![$($(String::from(stringify!($option))),*)?]
            }
        }

        $(#[$subcommand_arg_attr:meta])*
        #[derive(Args)]
        pub struct $subcommand_args {
            $(
                $(
                    $(#[$arg_attr])*
                    #[arg(help = $arg_des, required = true)]
                    pub $arg: $arg_type,
                )*
            )?
            $(
                $(
                    $(#[$option_attr])*
                    #[arg(help = $option_des, required = false)]
                    pub $option: $option_type,
                )*
            )?
        }
    };
}

#[macro_export]
macro_rules! subcommands_enum {
    (
        $($subcommands_enum_attr:meta),* $subcommands_enum:ident {
            $($($subcommand_attr:meta),* $subcommand:ident($subcommand_args:ident) => $subcommand_info:ident),*
        }
    ) => {
        $(#[$subcommands_enum_attr]),*
        #[derive(Subcommand)]
        pub enum $subcommands_enum {
            $(
                $(#[$subcommand_attr]),*
                #[command(about = $subcommand_info.about, long_about = $subcommand_info.long_about)]
                $subcommand($subcommand_args)
            ),*
        }
    };
}


#[macro_export]
macro_rules! cli_args {
    (

    ) => {

    };
}