use cursive::theme::{Theme, BorderStyle, Palette};
use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};

pub fn make_dark_theme() -> Theme {
    let mut palette = Palette::default();
    let colors = vec![
        (Background, Dark(Black)),
        (Shadow, Dark(Black)),
        (View, Light(Black)),
        (Primary, Light(White)),
        (Tertiary, Dark(Yellow)),
        (TitlePrimary, Dark(Yellow)),
        (TitleSecondary, Dark(Yellow)),
        (TitleSecondary, Dark(Yellow)),
        (Highlight, Dark(White)),
        (HighlightInactive, Dark(Black)),
        (HighlightText, Dark(White)),
    ];
    palette.extend(colors);
    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: palette,
    }
}