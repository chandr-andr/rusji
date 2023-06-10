use cursive::Cursive;

pub struct Button<'a> {
    pub keyboard_key: char,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

impl<'a> Button<'a> {
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
