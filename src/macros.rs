#[macro_export]
macro_rules! pub_struct {
    {
        $(#[$struct_attr:meta])*
        struct $name:ident$(<$T:ident>)? {
            $(
                $(#[$field_attr:meta])*
                $field:ident: $field_type:ty
            ),*
        }
    } => {
        $(#[$struct_attr])*
        pub struct $name$(<$T>)? {
            $(
                $(#[$field_attr])*
                pub $field: $field_type
            ),*
        }
    }
}

#[macro_export]
macro_rules! arc {
    [$($values:expr),*] => {
       Arc::new([$($values),*])
    };
}
