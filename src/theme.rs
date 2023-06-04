use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
use cursive::theme::{BorderStyle, Palette, Theme};

pub fn make_dark_theme() -> Theme {
    let mut palette = Palette::default();
    let colors = vec![
        (Background, TerminalDefault),
        (Shadow, Dark(Black)),
        (View, TerminalDefault),
        (Primary, Light(White)),
        (Tertiary, Dark(Green)),
        (TitlePrimary, Dark(Green)),
        (TitleSecondary, Dark(Green)),
        (Highlight, Dark(Blue)),
        (HighlightInactive, TerminalDefault),
        (HighlightText, TerminalDefault),
    ];
    palette.extend(colors);
    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette,
    }
}
