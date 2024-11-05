#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_attr:meta])*
        struct $name:ident {
            $($(#[$field_attr:meta])*
            $field:ident: $t:ty,)*
        }
    ) => {
        $(#[$struct_attr])*
        pub struct $name {
            $($(#[$field_attr])*
            pub $field: $t,)*
        }
    }
}

#[macro_export]
macro_rules! cli_interface {
    (
        $subcommands:ident {
            $($subcommand:ident $(#[$subcommand_attr:meta])* {
                about => $about:expr,
                long_about => $long_about:expr,
                flags {
                    $($flag:ident $(#[$flag_attr:meta])*: $flag_type:ty => $flag_des:expr $(=> $flag_long_des:expr)?),*
                }
                $(,enums {
                    $($enum_name:ident $(#[$enum_attr:meta])* { $($enum:ident $(=> $enum_des:expr)?),* }),*
                })?
            }),*
        }
    ) => {
        $(
            $(
                $(#[$enum_attr])*
                #[derive(Clone, ValueEnum)]
                enum $enum_name { 
                    $(
                        $(#[doc = enum_des])?
                        $enum
                    ),* 
                }
            )*
        )?

        #[derive(Subcommand)]
        pub enum $subcommands {
            $(
                $(#[$subcommand_attr])*
                #[command(about = $about, long_about = $long_about)]
                $subcommand {
                    $(
                        $(#[$flag_attr])*
                        #[arg(help = $flag_des)]
                        $(#[arg(long_help = $flag_long_des)])?
                        $flag: $flag_type
                    ),*
                }
            ),*
        }
    };
}
