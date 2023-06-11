use cursive::{event::Event, Cursive};

use super::buttons_variants::ButtonVariant;

/// Every callback must have display text.
pub trait CallbackText {
    fn display_text(&self) -> String;
}

/// Trait for all callbacks with button.
///
/// It means that you can click on this button or press callback key.
/// Usually using in SelectView.
pub trait ClickableCallback<'a, Variant, CursiveEvent>: CallbackText
where
    Variant: ButtonVariant<'a>,
    CursiveEvent: Into<Event>,
{
    fn new<ToStr>(
        variant: Variant,
        event: CursiveEvent,
        name: ToStr,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        ToStr: Into<&'a str>;
}

/// Trait for callback with small description.
///
/// It's not clickable, you can call callback only with key.
pub trait ShowOnlyCallback<'a, CursiveEvent>: CallbackText
where
    CursiveEvent: Into<Event>,
{
    fn new<ToStr>(
        event: CursiveEvent,
        name: ToStr,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        ToStr: Into<&'a str>;
}

/// Struct for clickable callbacks.
///
/// This struct must be used in SelectView.
pub struct CallbackWithButton<'a, Variant>
where
    Variant: ButtonVariant<'a>,
{
    pub variant: Variant,
    pub event: char,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

impl<'a, Variant> CallbackText for CallbackWithButton<'a, Variant>
where
    Variant: ButtonVariant<'a>,
{
    fn display_text(&self) -> String {
        let variant_in_text: &str = self.variant.into();
        format!("{} - {}", self.event, variant_in_text)
    }
}

impl<'a, Variant> ClickableCallback<'a, Variant, char>
    for CallbackWithButton<'a, Variant>
where
    Variant: ButtonVariant<'a>,
{
    fn new<ToStr>(
        variant: Variant,
        event: char,
        name: ToStr,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        ToStr: Into<&'a str>,
    {
        Self {
            variant,
            event,
            name: name.into(),
            action_fn,
        }
    }
}

/// Struct for not clickable callbacks.
///
/// This struct must be used in TextView or alternative.
pub struct StaticCallback<'a, CursiveEvent>
where
    CursiveEvent: Into<Event>,
{
    pub event: CursiveEvent,
    pub name: &'a str,
    pub action_fn: fn(&mut Cursive),
}

impl<'a, CursiveEvent> CallbackText for StaticCallback<'a, CursiveEvent>
where
    CursiveEvent: Into<Event>,
{
    fn display_text(&self) -> String {
        self.name.into()
    }
}

impl<'a, CursiveEvent> ShowOnlyCallback<'a, CursiveEvent>
    for StaticCallback<'a, CursiveEvent>
where
    CursiveEvent: Into<Event>,
{
    fn new<ToStr>(
        event: CursiveEvent,
        name: ToStr,
        action_fn: fn(&mut Cursive),
    ) -> Self
    where
        Self: Sized,
        ToStr: Into<&'a str>,
    {
        Self {
            event,
            name: name.into(),
            action_fn,
        }
    }
}
