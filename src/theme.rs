use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive::theme::{BorderStyle, Palette, Theme};

pub fn make_dark_theme() -> Theme {
    let mut palette = Palette::default();
    let colors = vec![
        (Background, TerminalDefault),
        (Shadow, Dark(Black)),
        (View, TerminalDefault),
        (Primary, Light(White)),
        (Tertiary, Dark(Yellow)),
        (TitlePrimary, Dark(Yellow)),
        (TitleSecondary, Dark(Yellow)),
        (Highlight, Dark(Blue)),
        (HighlightInactive, TerminalDefault),
        (HighlightText, Dark(White)),
    ];
    palette.extend(colors);
    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette,
    }
}
