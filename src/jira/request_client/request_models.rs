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

    pub fn add_story_points_and_return_as_string(
        self,
        new_story_points: usize,
        story_point_field_id: String,
    ) -> String {
        let mut value_data = self.data_as_value();
        let fields = value_data.get_mut("fields").unwrap();

        if let serde_json::Value::Object(fields_data) = fields {
            let mut new_field = serde_json::Map::new();
            new_field.insert(
                story_point_field_id,
                serde_json::Value::Number(serde_json::Number::from(
                    new_story_points,
                )),
            );
            fields_data.append(&mut new_field);
        }

        serde_json::to_string(&value_data).unwrap()
    }

    fn data_as_value(self) -> serde_json::Value {
        let str_data: String = serde_json::to_string(&self).unwrap();
        serde_json::from_str(str_data.as_str()).unwrap()
    }
}
