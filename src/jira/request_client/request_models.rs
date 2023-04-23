use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct IssueTransitionsReqData {
    transition: Option<IssueTransitionData>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct IssueTransitionData {
    id: usize,
}

impl IssueTransitionsReqData {
    pub fn new() -> Self {
        IssueTransitionsReqData {
            transition: Default::default(),
        }
    }

    pub fn add_transition_data(mut self, transition_id: usize) -> Self {
        self.transition =
            Option::Some(IssueTransitionData { id: transition_id });
        self
    }
}
