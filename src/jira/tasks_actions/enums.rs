pub enum Actions {
    StatusChange,
    ChangeExecutor,
    ChangeRelease,
    NotState,
}

impl From<&str> for Actions {
    fn from(action: &str) -> Self {
        match action {
            "Change status" => Actions::StatusChange,
            "Change executor" => Actions::ChangeExecutor,
            "Change release" => Actions::ChangeRelease,
            _ => Actions::NotState,
        }
    }
}

impl From<Actions> for &str {
    fn from(action: Actions) -> Self {
        match action {
            Actions::StatusChange => "Change status",
            Actions::ChangeExecutor => "Change executor",
            Actions::ChangeRelease => "Change release",
            _ => "NotState",
        }
    }
}

impl Actions {
    pub fn get_actions() -> Vec<&'static str> {
        vec![
            Self::StatusChange.into(),
            Self::ChangeExecutor.into(),
            Self::ChangeRelease.into(),
        ]
    }
}