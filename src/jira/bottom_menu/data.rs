use cursive::Cursive;

use crate::jira::menu::views::MenuView;

pub struct BottomButtons<'a> {
    pub buttons: Vec<BottomButton<'a>>,
}

pub struct BottomButton<'a> {
    pub keyboard_key: char,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

impl<'a> BottomButton<'a> {
    /// Create new button.
    pub fn new<S>(
        keyboard_key: char,
        name: S,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        S: Into<&'a str>,
    {
        Self {
            keyboard_key,
            name: name.into(),
            action_fn,
        }
    }

    /// Return button's full name for the bottom view.
    pub fn full_name(&self) -> String {
        format!("{} - {}", self.keyboard_key, self.name)
    }
}

impl<'a> BottomButtons<'a> {
    pub fn new() -> Self {
        let mut buttons: Vec<BottomButton> = Vec::default();
        buttons.push(BottomButton::new(
            'm',
            "menu",
            |cursive: &mut Cursive| {
                let menu = MenuView::new(cursive);
                cursive.add_layer(menu);
            },
        ));
        buttons.push(BottomButton::new(
            'q',
            "quit",
            |cursive: &mut Cursive| {
                cursive.quit();
            },
        ));

        Self { buttons }
    }

    pub fn buttons_text(&self) -> String {
        self.buttons
            .iter()
            .map(|button| format!("| {} |", button.full_name()))
            .collect()
    }
}
