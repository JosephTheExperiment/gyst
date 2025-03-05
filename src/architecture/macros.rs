#[macro_export]
macro_rules! mod_all {
    ($($visibility:vis $mod:ident),*) => {
        $($visibility mod $mod;)*
    };
}

// Stylized string
#[macro_export]
macro_rules! SS {
    ($fg:ident, $bg:ident, $ul:ident, $($attr:ident),* /$str:literal) => {
        { build_stylized_string(Some(Color::$fg), Some(Color::$bg), Some(Color::$ul), vec![$(Attribute::$attr:ident),*]), String::from($str) }
    };
    ($fg:ident, $bg:ident /$str:literal) => {
        { build_stylized_string(Some(Color::$fg), Some(Color::$bg), None, vec![], String::from($str)) }
    };
    ($fg:ident /$str:literal) => {
        { build_stylized_string(Some(Color::$fg), None, None, vec![], String::from($str)) }
    };
    (attr: $ul:ident, $($attr:ident),* /$str:literal) => {
        { build_stylized_string(None, None, Some(Color::$ul), vec![$(Attribute::$attr:ident),*]), String::from($str) }
    }
}

#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_attr:meta])*
        struct $name:ident$(<$T:ident>)? {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        pub struct $name$(<$T>)? {
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
