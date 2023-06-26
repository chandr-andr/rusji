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

impl<'a> AssigneeData<'a> {
    fn new(assignee_username: &'a str) -> Self {
        Self {
            name: assignee_username,
        }
    }
}

#[derive(Default, Serialize)]
struct IssueFieldsReqData<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee: Option<AssigneeData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(alias = "Story Points")]
    story_points: Option<usize>,
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
        let assignee_data = Some(AssigneeData::new(assignee_username));
        self.fields.assignee = assignee_data;
    }
}
