#[macro_export]
macro_rules! subcommand_arg_build {
    (
        $subcommand_args:ident $($subcommand_args_attr:meta),* {
            fn_des => $fn_des:ident,
            about => $about:expr,
            long_about => $long_about:expr
            $(,
                args {
                    $($arg:ident $($arg_attr:meta),*: $arg_type:ty => $arg_des:expr),*
                }
            )?
            $(,
                options {
                    $($option:ident $($option_attr:meta),*: $option_type:ty => $option_des:expr),*
                }
            )?
            $(,
                enums {
                    $($enum_name:ident $($enum_attr:meta),* { $($enum:ident => $enum_des:expr),* }),*
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

        pub fn fn_des() -> (String, String) { ($about.to_string(), $long_about.to_string()) }

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
macro_rules! cli_build {
    (
        $cli:ident $($cli_attr:meta),* {
            $(
                subcommands $subcommands:ident $($subcommands_attr:meta),*
            ,)?
            $(
                args_flags {
                    $($flag:ident $($flag_attr:meta),*: $flag_type:ty => $flag_des:expr $(=> $flag_long_des:expr)?),*
                }
            )?
            $(,
                options {
                    $($option:ident $($option_attr:meta),*: $option_type:ty => $option_des:expr $(=> $option_long_des:expr)?),*
                }
            )?
            $(,
                enums {
                    $($enum_name:ident $($enum_attr:meta),* { $($enum:ident $(=> $enum_des:expr)?),* }),*
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
                        $(#[doc = $enum_des])?
                        $enum
                    ),*
                }
            )*
        )?

        #[derive(Parser)]
        $(#[$cli_attr])*
        pub struct $cli {
            $(
                $(#[$subcommands_attr])*
                #[command(subcommand)]
                pub command: $subcommands,
            )?
            $(
                $(
                    $(#[$flag_attr])*
                    #[arg(help = $flag_des, required = true)]
                    $(#[arg(long_help = $flag_long_des)])?
                    pub $flag: $flag_type,
                )*
            )?
            $(
                $(
                    $(#[$option_attr])*
                    #[arg(help = $option_des, required = false)]
                    $(#[arg(long_help = $option_long_des)])?
                    pub $option: $option_type,
                )*
            )?
        }
    };
}
