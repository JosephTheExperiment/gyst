#[macro_export]
macro_rules! subcommand {
    (
        $(#[$subcommand_args_attr:meta])* $subcommand:ident {
            about => $about:expr;
            long_about => $long_about:expr;
            $(
                args { $(
                    $(#[$arg_attr:meta])* $arg:ident : $arg_type:ty => $arg_des:expr
                ),* };
            )?
            $(
                options { $(
                    $(#[$option_attr:meta])* $option:ident : $option_type:ty => $option_des:expr
                ),* };
            )?
            $(
                enums { $(
                    $(#[$enum_attr:meta])* $enum:ident { $($variant:ident => $enum_des:expr),* }
                ),* };
            )?
        }
    ) => {
        $(
            $(
                #[derive(Clone, ValueEnum)]
                $(#[$enum_attr])*
                pub enum $enum { $(
                    #[clap(about = $enum_des)] $variant
                ),* }
            )*
        )?

        paste! {            
            #[derive(Args)]
            #[command(about = $about, long_about = $long_about)]
            $(#[$subcommand_args_attr:meta])*
            pub struct [<$subcommand Args>] {
                $(
                    $(
                        #[arg(help = $arg_des, required = true)]
                        $(#[$arg_attr])*
                        pub $arg: $arg_type,
                    )*
                )?
                $(
                    $(
                        #[arg(help = $option_des, required = false)]
                        $(#[$option_attr])*
                        pub $option: $option_type,
                    )*
                )?
            }
        }
    };
}

#[macro_export]
macro_rules! cli {
    (
        $(#[$cli_attr:meta])* $cli:ident {
            $(
                $(#[$subcommands_enum_attr:meta])* $subcommands_enum:ident { $($subcommand:ident),* };
            )?
            $(
                args { $(
                    $(#[$arg_attr:meta])* $arg:ident : $arg_type:ty => $arg_des:expr
                ),* };
            )?
            $(
                options { $(
                    $(#[$option_attr:meta])* $option:ident : $option_type:ty => $option_des:expr
                ),* };
            )?
            $(
                enums { $(
                    $(#[$enum_attr:meta])* $enum:ident { $($variant:ident => $enum_des:expr),* }
                ),* };
            )?
        }
    ) => {
        $(
            paste! {
                #[derive(Subcommand)]
                $(#[$subcommands_enum_attr])*
                pub enum $subcommands_enum { $(
                    $subcommand([<$subcommand Args>])
                ),* }
            }
        )?

        $(
            $(
                #[derive(Clone, ValueEnum)]
                $(#[$enum_attr])*
                pub enum $enum { $(
                    #[doc = $enum_des] $variant
                ),* }
            )*
        )?

        #[derive(Parser)]
        $(#[$cli_attr])*
        pub struct $cli {
            $( 
                #[command(subcommand)] pub command: $subcommands_enum,
            )?
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