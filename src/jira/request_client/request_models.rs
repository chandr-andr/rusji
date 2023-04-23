use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct IssueTransitionsReqData<'a> {
    transition: Option<IssueTransitionData<'a>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct IssueTransitionData<'a> {
    id: &'a str,
}

impl<'a> IssueTransitionsReqData<'a> {
    pub fn new() -> Self {
        IssueTransitionsReqData {
            transition: Default::default(),
        }
    }

    pub fn add_transition_data(mut self, transition_id: &'a str) -> Self {
        self.transition =
            Option::Some(IssueTransitionData { id: transition_id });
        self
    }
}
