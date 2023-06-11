use cursive::{event::Event, Cursive};

pub trait Button<'a, CursiveEvent> {
    /// Create new button.
    fn new<S>(
        event: CursiveEvent,
        name: S,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        S: Into<&'a str>,
        CursiveEvent: Into<CursiveEvent>;

    /// Return button's full name for the bottom view.
    fn full_name(&self) -> String;
}

pub struct CustomizableButton<'a> {
    pub event: char,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

struct StaticButton<'a, CursiveEvent>
where
    CursiveEvent: Into<Event>,
{
    pub event: CursiveEvent,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

impl<'a> Button<'a, char> for CustomizableButton<'a> {
    fn new<S>(event: char, name: S, action_fn: fn(&mut Cursive)) -> Self
    where
        Self: Sized,
        S: Into<&'a str>,
        char: Into<char>,
    {
        Self {
            event: event,
            name: name.into(),
            action_fn: action_fn,
        }
    }

    fn full_name(&self) -> String {
        format!("{} - {}", self.event, self.name)
    }
}

impl<'a, CursiveEvent> Button<'a, CursiveEvent>
    for StaticButton<'a, CursiveEvent>
where
    CursiveEvent: Into<Event>,
{
    fn new<S>(
        event: CursiveEvent,
        name: S,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        S: Into<&'a str>,
        CursiveEvent: Into<CursiveEvent>,
    {
        Self {
            event: event,
            name: name.into(),
            action_fn: action_fn,
        }
    }

    fn full_name(&self) -> String {
        self.name.into()
    }
}
