use std::sync::{Arc, RwLock};

use cursive::Cursive;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    errors::RusjiResult, jira_data::JiraData,
    request_client::request_client::RequestClient,
};

/// JiraIssues holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraIssues {
    issues: Vec<JiraTask>,
}

impl IntoIterator for JiraIssues {
    type Item = JiraTask;
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
pub struct JiraTask {
    pub id: String,
    #[serde(alias = "self")]
    pub link: String,
    pub key: String,
    pub description: String,
    pub summary: String,
    pub status: JiraTaskStatus,
    pub transitions: Option<IssueTransitions>,
}

/// Creates custom Deserialize for JiraTask.
///
/// It is used because there is no necessities to store
/// real json structure.
/// No need to have a lot of nested structs.
impl<'de> Deserialize<'de> for JiraTask {
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
            status: JiraTaskStatus,
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

        Ok(JiraTask {
            id: task.id,
            link: task.link,
            key: task.key,
            description: task.rendered_fields.description,
            summary: task.fields.summary,
            status: task.fields.status,
            transitions: Default::default(),
        })
    }
}

impl JiraTask {
    /// Creates new instance of JiraTask.
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
    pub fn all_transactions_name(&self) -> Vec<&str> {
        self.transitions
            .iter()
            .map(|issue_transaction| issue_transaction.name.as_str())
            .collect()
    }
}

/// Struct for single task status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JiraTaskStatus {
    pub id: String,
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    name: String,
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

        serde_json::from_str::<JiraTask>(json_task_str).unwrap();
    }
}
