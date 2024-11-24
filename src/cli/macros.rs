// My beautiful macro WILL NOT be deleted
#[macro_export]
macro_rules! pub_struct {
    (
        $(#[$struct_attr:meta])*
        struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $t:ty,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        pub struct $name {
            $(
                $(#[$field_attr])*
                pub $field: $t,
            )*
        }
    }
}

#[macro_export]
macro_rules! subcommands_build_interface {
    (
        $subcommands:ident {
            $(
                $subcommand:ident $($subcommand_attr:meta),* {
                    about => $about:expr,
                    long_about => $long_about:expr,
                    $(    
                        flags {
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
            ),*
        }
    ) => {
        $(
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
        )*

        #[derive(Subcommand)]
        pub enum $subcommands {
            $(
                $(#[$subcommand_attr])*
                #[command(about = $about, long_about = $long_about)]
                $subcommand {
                    $(
                        $(
                            $(#[$flag_attr])*
                            #[arg(help = $flag_des, required = true)]
                            $(#[arg(long_help = $flag_long_des)])?
                            $flag: $flag_type,
                        )*
                    )?
                    $(
                        $(
                            $(#[$option_attr])*
                            #[arg(help = $option_des, required = false)]
                            $(#[arg(long_help = $option_long_des)])?
                            $option: $option_type,
                        )*
                    )?
                }
            ),*
        }
    };
}

#[macro_export]
macro_rules! cli_build_interface {
    (
        $cli:ident $($cli_attr:meta),* {
            $(
                subcommands $subcommands:ident $($subcommands_attr:meta),*
            ,)?
            $(
                flags {
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