use cursive::utils::{markup::StyledString};

enum Actions {
    StatusChange,
}

impl From<Actions> for StyledString {
    fn from(action: Actions) -> Self {
        match action {
            StatusChange => StyledString::plain("StatusChange"),
        }
    }
}