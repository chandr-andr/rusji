use std::fmt::Display;

pub trait ButtonVariant<'a>: Into<&'a str> + From<&'a str> + Display {}
