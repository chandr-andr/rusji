use std::str::FromStr;

pub trait ButtonVariant<'a>: Into<&'a str> + FromStr + Copy {}
