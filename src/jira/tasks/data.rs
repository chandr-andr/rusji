use std::collections::BTreeMap as Map;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    errors::RusjiResult, jira::tasks_actions::data::JiraUser,
    request_client::request_client::RequestClient,
};

/// JiraIssues holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraIssues {
    issues: Vec<JiraIssue>,
}

impl IntoIterator for JiraIssues {
    type Item = JiraIssue;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.issues.into_iter()
    }
}

impl JiraIssues {
    /// Creates new instance of JiraIssues.
    ///
    /// Makes request to get tasks for project and parses the response.
    /// If request failed return error.
    pub fn new(
        request_client: Arc<RwLock<RequestClient>>,
        project_name: &str,
    ) -> RusjiResult<Self> {
        let response = request_client
            .read()
            .unwrap()
            .get_tasks_from_project(project_name)?;

        let resp_text = response.get_body();
        let tasks = serde_json::from_str::<Self>(resp_text)?;
        Ok(tasks)
    }
}

/// Struct for single task in Jira.
#[derive(Serialize, Debug, Clone)]
pub struct JiraIssue {
    pub id: String,
    #[serde(alias = "self")]
    pub link: String,
    pub key: String,
    pub description: String,
    pub summary: String,
    pub status: JiraIssueStatus,
    pub transitions: Option<IssueTransitions>,
    pub assignee: Option<JiraUser>,
}

/// Creates custom Deserialize for JiraTask.
///
/// It is used because there is no necessities to store
/// real json structure.
/// No need to have a lot of nested structs.
impl<'de> Deserialize<'de> for JiraIssue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Debug)]
        struct Task {
            id: String,
            #[serde(alias = "self")]
            link: String,
            key: String,
            fields: Fields,
            #[serde(
                default = "default_rendered_fields",
                alias = "renderedFields"
            )]
            rendered_fields: RenderedFields,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Fields {
            summary: String,
            status: JiraIssueStatus,
            assignee: Option<JiraUser>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct RenderedFields {
            description: String,
        }

        fn default_rendered_fields() -> RenderedFields {
            RenderedFields {
                description: "No description".to_string(),
            }
        }

        let task = Task::deserialize(deserializer)?;

        Ok(JiraIssue {
            id: task.id,
            link: task.link,
            key: task.key,
            description: task.rendered_fields.description,
            summary: task.fields.summary,
            status: task.fields.status,
            transitions: Default::default(),
            assignee: task.fields.assignee,
        })
    }
}

impl JiraIssue {
    /// Create new instance of JiraIssue.
    ///
    /// Makes request to Jira API.
    /// Can return `RusjiError`.
    pub fn new(
        request_client: Arc<RwLock<RequestClient>>,
        issue_key: &str,
    ) -> RusjiResult<Self> {
        let response = request_client.read().unwrap().get_task(issue_key)?;
        let resp_text = response.get_body();
        let task = serde_json::from_str::<Self>(resp_text)?;
        Ok(task)
    }

    /// Add transitions to the JiraIssue instance.
    ///
    /// It is necessary because issue status can be changed in time,
    /// so here transitions get in real time.
    pub fn add_transitions(
        &mut self,
        request_client: Arc<RwLock<RequestClient>>,
    ) {
        let response = request_client
            .read()
            .unwrap()
            .get_issue_transitions(&self.key)
            .unwrap();

        let available_transactions =
            serde_json::from_str::<IssueTransitions>(response.get_body())
                .unwrap();
        self.transitions = Option::Some(available_transactions);
    }
}

// Model for all tasks transactions
// that available at the moment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueTransitions {
    transitions: Vec<IssueTransition>,
}

/// Model for single transaction data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueTransition {
    pub id: String,
    pub name: String,
}

impl IssueTransitions {
    /// Return name for all transactions.
    pub fn all_transitions_name(&self) -> Vec<&str> {
        self.transitions
            .iter()
            .map(|issue_transaction| issue_transaction.name.as_str())
            .collect()
    }

    pub fn get_transitions_id_by_name(&self, transition_name: &str) -> &str {
        for transition in &self.transitions {
            if transition.name == transition_name {
                return transition.id.as_str();
            }
        }
        "0"
    }
}

/// Struct for single task status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JiraIssueStatus {
    pub id: String,
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    pub name: String,
}

/// Struct for single task category.
#[derive(Deserialize, Serialize, Debug)]
struct StatusCategory {
    #[serde(alias = "self")]
    link: String,
    id: u8,
    key: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IssueMetaData {
    #[serde(flatten)]
    other: Map<String, Value>,
}

impl IssueMetaData {
    pub fn get_story_points_field_id(&self) -> Option<String> {
        let fields_data = self.other.get("fields").unwrap();

        match fields_data {
            Value::Object(fields_data) => {
                for (data_key, data_value) in fields_data {
                    if !data_key.contains("customfield") {
                        continue;
                    }
                    match data_value {
                        Value::Object(data) => {
                            let customfield_name = data.get("name");
                            if let Some(name) = customfield_name {
                                if let Value::String(name) = name {
                                    if name.contains("Story Points") {
                                        let field_id = data.get("fieldId");
                                        if let Some(field_id) = field_id {
                                            match field_id {
                                                Value::String(field_id) => {
                                                    return Some(
                                                        field_id.clone(),
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_task() {
        let json_task_str = r#"
        {
            "expand": "renderedFields,names,schema,operations,editmeta,changelog,versionedRepresentations",
            "id": "299756",
            "link": "https://link.com",
            "key": "FRE-39",
            "fields": {
                "issuetype": {
                    "self": "https://link.com",
                    "id": "10001",
                    "description": "Created by Jira Software - do not edit or delete. Issue type for a user story.",
                    "iconUrl": "https://link.com",
                    "name": "Story",
                    "subtask": false
                },
                "description": "test description",
                "summary": "test summary",
                "status": {
                    "self": "https://link.com",
                    "description": "Задача завершена",
                    "iconUrl": "https://link.com",
                    "name": "DONE",
                    "id": "10104",
                    "statusCategory": {
                        "self": "https://link.com",
                        "id": 3,
                        "key": "done",
                        "colorName": "green",
                        "name": "Выполнено"
                    }
                }
            },
            "renderedFields": {
                "description": "test"
            }
        }
        "#;

        serde_json::from_str::<JiraIssue>(json_task_str).unwrap();
    }

    #[test]
    fn test_deserialize_issue_metadata() {
        let json_issue_metadata_str = r#"
        {
            "fields": {
              "customfield_10101": {
                "required": false,
                "schema": {
                  "type": "any",
                  "custom": "com.pyxis.greenhopper.jira:gh-epic-link",
                  "customId": 10101
                },
                "name": "Epic Link",
                "fieldId": "customfield_10101",
                "operations": [
                  "set"
                ]
              },
              "customfield_10100": {
                "required": false,
                "schema": {
                  "type": "array",
                  "items": "string",
                  "custom": "com.pyxis.greenhopper.jira:gh-sprint",
                  "customId": 10100
                },
                "name": "Sprint",
                "fieldId": "customfield_10100",
                "operations": [
                  "set"
                ]
              },
              "customfield_10106": {
                "required": false,
                "schema": {
                  "type": "number",
                  "custom": "com.atlassian.jira.plugin.system.customfieldtypes:float",
                  "customId": 10106
                },
                "name": "Story Points",
                "fieldId": "customfield_10106",
                "operations": [
                  "set"
                ]
              }
            }
          }
        "#;

        let issue_meta_data =
            serde_json::from_str::<IssueMetaData>(json_issue_metadata_str)
                .unwrap();

        let story_point_field_id =
            issue_meta_data.get_story_points_field_id().unwrap();

        assert!("customfield_10106" == story_point_field_id);
    }
}
