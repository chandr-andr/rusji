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
        Self {
            transition: Default::default(),
        }
    }

    pub fn add_transition_data(mut self, transition_id: &'a str) -> Self {
        self.transition =
            Option::Some(IssueTransitionData { id: transition_id });
        self
    }
}

#[derive(Default, Serialize, Deserialize)]
struct AssigneeData<'a> {
    name: &'a str,
}

#[derive(Default, Serialize)]
struct IssueFieldsReqData<'a> {
    assignee: AssigneeData<'a>,
}

#[derive(Serialize)]
pub(crate) struct IssuePropertiesReqData<'a> {
    fields: IssueFieldsReqData<'a>,
}

impl<'a> IssuePropertiesReqData<'a> {
    pub fn new() -> Self {
        Self {
            fields: Default::default(),
        }
    }

    pub fn set_assignee(&mut self, assignee_username: &'a str) {
        self.fields.assignee.name = assignee_username;
    }
}
