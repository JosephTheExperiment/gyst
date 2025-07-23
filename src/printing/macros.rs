// Stylized string
#[macro_export]
macro_rules! SS {
    ($(fg:$fg:ident;)? $(bg:$bg:ident;)? $(ul:$ul:ident;)? $(attr:$($attr:ident),*)? => $str:expr) => {
        {
            let mut content_style = ContentStyle::new();
            $(content_style.foreground_color = Some(Color::$fg);)?
            $(content_style.background_color = Some(Color::$bg);)?
            $(content_style.underline_color = Some(Color::$ul);)?
            $($(content_style.attributes.set(Attribute::$attr);)*)?
            StylizedString(content_style, String::from($str))
        }
    };

    ($fg:ident => $str:expr) => { SS!(fg:$fg; => $str) };
    ($fg:ident, $bg:ident => $str:expr) => { SS!(fg:$fg; bg:$bg; => $str) };
    ($fg:ident, $bg:ident, $ul:ident, $($attr:ident),* => $str:expr) => {
        SS!(fg:$fg; bg:$bg; ul:$ul; attr:$($attr),* => $str)
    };
}
