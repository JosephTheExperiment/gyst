use anstyle::{AnsiColor, Color::Ansi, Style};

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Ansi(AnsiColor::Blue))),
        )
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Ansi(AnsiColor::Yellow))),
        )
        .literal(Style::new().fg_color(Some(Ansi(AnsiColor::Green))))
        .invalid(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Red))))
        .error(Style::new().bold().fg_color(Some(Ansi(AnsiColor::Red))))
        .valid(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Ansi(AnsiColor::White))))
}
