#[macro_export]
macro_rules! subcommand {
    (
        #[$($subcommand_args_attr:meta),*] $subcommand:ident {
            about => $about:expr,
            long_about => $long_about:expr
            $(,
                args { $(
                    #[$($arg_attr:meta),*] $arg:ident : $arg_type:ty => $arg_des:expr
                ),* }
            )?
            $(,
                options { $(
                    #[$($option_attr:meta),*] $option:ident : $option_type:ty => $option_des:expr
                ),* }
            )?
            $(,
                enums { $(
                    #[$($enum_attr:meta),*] $enum:ident { $($variant:ident => $enum_des:expr),* }
                ),* }
            )?
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                pub enum $enum { $(
                    #[doc = $enum_des] $variant
                ),* }
            )*
        )?

        paste! {            
            $(#[$subcommand_args_attr:meta])*
            #[derive(Args)]
            #[command(about = $about, long_about = $long_about)]
            pub struct [<$subcommand Args>] {
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
        #[$($subcommands_enum_attr:meta),*] $subcommands_enum:ident {
            $(#[$($subcommand_attr:meta),*] $subcommand:ident),*
        }
    ) => {
        paste! {
            $(#[$subcommands_enum_attr])*
            #[derive(Subcommand)]
            pub enum $subcommands_enum {
                $(
                    $(#[$subcommand_attr])*
                    $subcommand([<$subcommand Args>])
                ),*
            }
        }
    };
}


#[macro_export]
macro_rules! cli {
    (
        #[$($cli_attr:meta),*] $cli:ident {
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
                enums { $($($enum_attr:meta),* $enum:ident { $($variant:ident => $enum_des:expr),* }),* }
            )?
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                pub enum $enum {
                    $(#[doc = $enum_des] $variant),*
                }
            )*
        )?

        #[derive(Parser)]
        $(#[$cli_attr])*
        pub struct $cli {
            $( #[command(subcommand)] pub command: $subcommands_enum, )?
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