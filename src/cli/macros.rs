#[macro_export]
macro_rules! mod_all {
    ($($visibility:vis $mod:ident),*) => {
        $($visibility mod $mod;)*
    };
}

#[macro_export]
macro_rules! subcommand {
    (
        $(#[$subcommand_args_attr:meta])* $subcommand:ident {
            about => $about:expr;
            long_about => $long_about:expr;
            $(
                args { $(
                    $(#[$arg_attr:meta])* $arg:ident : $arg_type:ty => $arg_des:expr $(=> $arg_long_des:expr)?
                ),* };
            )?
            $(
                options { $(
                    $(#[$option_attr:meta])* $option:ident : $option_type:ty $(= $default:expr)? => $option_des:expr $(=> $option_long_des:expr)?
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
                #[clap(rename_all = "snake_case")]
                $(#[$enum_attr])*
                pub enum $enum { $(
                    #[clap(help = $enum_des)] $variant
                ),* }
            )*
        )?

        paste! {
            #[derive(Args)]
            #[command(about = $about, long_about = format!("{}\n\n{}", $about, $long_about))]
            $(#[$subcommand_args_attr:meta])*
            pub struct [<$subcommand Args>] {
                $(
                    $(
                        $(#[arg(long_help = format!("{}\n\n{}", $arg_des, $arg_long_des))])?
                        #[arg(help = $arg_des, required = true)]
                        $(#[$arg_attr])*
                        pub $arg: $arg_type,
                    )*
                )?
                $(
                    $(
                        $(#[arg(long_help = format!("{}\n\n{}", $option_des, $option_long_des))])?
                        $(#[arg(default_value_t = $default)])?
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
                    $(#[$arg_attr:meta])* $arg:ident : $arg_type:ty => $arg_des:expr $(=> $arg_long_des:expr)?
                ),* };
            )?
            $(
                options { $(
                    $(#[$option_attr:meta])* $option:ident : $option_type:ty $(= $default:expr)? => $option_des:expr $(=> $option_long_des:expr)?
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
                #[clap(rename_all = "snake_case")]
                $(#[$enum_attr])*
                pub enum $enum { $(
                    #[help = $enum_des] $variant
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
                    $(#[arg(long_help = format!("{}\n\n{}", $arg_des, $arg_long_des))])?
                    #[arg(help = $arg_des, required = true)]
                    $(#[$arg_attr])*
                    pub $arg: $arg_type,
                )*
            )?
            $(
                $(
                    $(#[arg(long_help = format!("{}\n\n{}", $option_des, $option_long_des))])?
                    $(#[arg(default_value_t = $default)])?
                    #[arg(help = $option_des, required = false)]
                    $(#[$option_attr])*
                    pub $option: $option_type,
                )*
            )?
        }
    };
}
