#[macro_export]
macro_rules! style {
    ($(fg:$fg:ident;)? $(bg:$bg:ident;)? $(ul:$ul:ident;)? $(attr:[$($attr:ident),*])?) => {
        {
            let mut content_style = ContentStyle::new();
            $(content_style.foreground_color = Some(Color::$fg);)?
            $(content_style.background_color = Some(Color::$bg);)?
            $(content_style.underline_color = Some(Color::$ul);)?
            $($(content_style.attributes.set(Attribute::$attr);)*)?
            content_style
        }
    };

    ($fg:ident) => { style!(fg:$fg;) };
    ($fg:ident, $bg:ident) => { style!(fg:$fg; bg:$bg;) };
    ($fg:ident, $bg:ident, $ul:ident, [$($attr:ident),*]) => {
        style!(fg:$fg; bg:$bg; ul:$ul; attr:[$($attr),*])
    };
}
