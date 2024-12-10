use paste::paste;

#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_attr:meta])*
        struct $struct:ident {
            $($(#[$field_attr:meta])* $field:ident: $t:ty),*
        }
    ) => {
        $(#[$struct_attr])*
        pub struct $struct {
            $($(#[$field_attr])* pub $field: $t),*
        }
    }
}

pub_struct!(
    struct SubcommandInfo {
        name: String,
        about: String,
        long_about: String,
        args: Vec<String>,
        options: Vec<String>
    }
);

#[macro_export]
macro_rules! subcommand_args {
    (
        $($subcommand_args_attr:meta),* $subcommand:ident {
            about => $about:expr,
            long_about => $long_about:expr
            $(,
                args { $($($arg_attr:meta),* $arg:ident : $arg_type:ty => $arg_des:expr),* }
            )?
            $(,
                options { $($($option_attr:meta),* $option:ident : $option_type:ty => $option_des:expr),* }
            )?
            $(,
                enums { $($($enum_attr:meta),* $enum_name:ident { $($enum:ident => $enum_des:expr),* }),* }
            )?
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                pub enum $enum_name {
                    $(#[doc = $enum_des] $enum),*
                }
            )*
        )?

        paste! {
            pub fn [<$subcommand _info()>] -> SubcommandInfo {
                SubcommandInfo {
                    name: String::from(stringify!($subcommand)),
                    about: $about.to_string(),
                    long_about: $long_about.to_string(),
                    args: vec![$( $(stringify!($arg).to_string()),* )?],
                    options: vec![$( $(stringify!($option).to_string()),* )?]
                }
            }

            $(#[$subcommand_arg_attr:meta])*
            #[derive(Args)]
            pub struct [<$subcommand _args()>] {
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
        }
    };
}

#[macro_export]
macro_rules! subcommands_enum {
    (
        $($subcommands_enum_attr:meta),* $subcommands_enum:ident {
            $($($subcommand_attr:meta),* $subcommand:ident),*
        }
    ) => {
        paste! {
            $(#[$subcommands_enum_attr]),*
            #[derive(Subcommand)]
            pub enum $subcommands_enum {
                $(
                    $(#[$subcommand_attr]),*
                    #[command(about = [<$subcommand _info>].about, long_about = [<$subcommand _info>].long_about)]
                    $subcommand([<$subcommand _args>])
                ),*
            }
        }
    };
}


#[macro_export]
macro_rules! cli {
    (
        $($cli_attr:meta),* $cli:ident {
            $(
                subcommands : $subcommands_enum:ident 
            )?
            $(,
                args { $($($arg_attr:meta),* $arg:ident : $arg_type:ty => $arg_des:expr),* }
            )?
            $(,
                options { $($($option_attr:meta),* $option:ident : $option_type:ty => $option_des:expr),* }
            )?
            $(,
                enums { $($($enum_attr:meta),* $enum_name:ident { $($enum:ident => $enum_des:expr),* }),* }
            )?
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                pub enum $enum_name {
                    $(#[doc = $enum_des] $enum),*
                }
            )*
        )?

        #[derive(Parser)]
        struct $cli {
            $( #[command(subcommand)] command: $subcommands_enum, )?
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