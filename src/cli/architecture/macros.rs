#[macro_export]
macro_rules! mod_all {
    ($($visibility:vis $mod:ident),*) => {
        $($visibility mod $mod;)*
    };
}

#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_attr:meta])*
        struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        pub struct $name {
            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
        }
    }
}

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

#[macro_export]
macro_rules! cli {
    (
        $(#[$cli_attr:meta])* $cli:ident {
            $(
                $(#[$subcommands_enum_attr:meta])* $subcommands_enum:ident { $($subcommand:ident),* };
            )?
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
                pub enum $enum { $($variant),* }
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
                    $(#[arg(default_value = $default_arg)])?
                    $(#[$arg_attr:meta])*
                    pub $arg: $arg_type,
                )*
            )?
        }
    };
}
