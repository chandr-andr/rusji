use cursive::Cursive;

pub struct Buttons<'a> {
    buttons: Vec<Button<'a>>,
}

pub struct Button<'a> {
    keyboard_key: &'a str,
    name: &'a str,
    action_fn: fn(&Cursive),
}

impl<'a> Button<'a> {
    pub fn new<S>(keyboard_key: S, name: S, action_fn: fn(&Cursive)) -> Self
    where
        S: Into<&'a str>,
    {
        Self {
            keyboard_key: keyboard_key.into(),
            name: name.into(),
            action_fn: action_fn,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} - {}", self.keyboard_key, self.name)
    }
}

fn test(cursive: &Cursive) {}

impl<'a> Buttons<'a> {
    pub fn new() -> Self {
        let mut buttons: Vec<Button> = Vec::default();

        let m_button = Button::new("m", "menu", test);

        buttons.push(m_button);
        Self { buttons: buttons }
    }
}
