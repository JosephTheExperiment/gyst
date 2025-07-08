#[macro_export]
macro_rules! subcommand {
    (
        $(#[$subcommand_args_attr:meta])* $subcommand:ident {
            $(
                args { $(
                    $(#[$arg_attr:meta])* $arg:ident: $arg_type:ty $(= $default_arg:expr)?
                ),* };
            )?
            $(
                enums { $(
                    $(#[$enum_attr:meta])* $enum:ident { $($variant:ident),* }
                ),* };
            )?
        }
    ) => {
        $(
            $(
                #[derive(Clone, ValueEnum)]
                #[clap(rename_all = "snake_case")]
                $(#[$enum_attr])*
                pub enum $enum { $($variant),* }
            )*
        )?

        paste! {
            #[derive(Args)]
            $(#[$subcommand_args_attr:meta])*
            pub struct [<$subcommand Args>] {
                $(
                    $(
                        $(#[arg(default_value = $default_arg)])?
                        $(#[$arg_attr])*
                        pub $arg: $arg_type,
                    )*
                )?
            }
        }
    };
}
