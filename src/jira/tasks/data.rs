use std::{sync::{Arc, RwLock}};

use serde::{Deserialize, Deserializer, Serialize};

use crate::{errors::RusjiResult, request_client::RequestClient};

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
    pub issuetype: JiraTaskType,
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
            #[serde(default = "default_rendered_fields", alias = "renderedFields")]
            rendered_fields: RenderedFields,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Fields {
            summary: String,
            status: JiraTaskStatus,
            issuetype: JiraTaskType,
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
            issuetype: task.fields.issuetype,
        })
    }
}

impl JiraTask {
    /// Creates new instance of JiraTask.
    ///
    /// Makes request to Jira API.
    /// Can return `RusjiError`.
    pub fn new(request_client: Arc<RwLock<RequestClient>>, task_key: &str) -> RusjiResult<Self> {
        let response = request_client.read().unwrap().get_task(task_key)?;
        let resp_text = response.get_body();
        let task = serde_json::from_str::<Self>(resp_text)?;
        Ok(task)
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JiraTaskType {
    id: String,
    #[serde(alias = "self")]
    link: String,
    description: String,
    pub name: String,
    subtask: bool,
}

/// Struct for task types.
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskTypes(Vec<TaskType>);

impl TaskTypes {
    /// Creates new instance of task types.
    ///
    /// Makes request to Jira API.
    /// Can return RusjiError.
    pub fn new(
        request_client: Arc<RwLock<RequestClient>>,
        project_name: &str,
    ) -> RusjiResult<Self> {
        let response = request_client
            .read()
            .unwrap()
            .get_task_statuses(project_name)?;
        let resp_text = response.get_body();
        let statuses = serde_json::from_str::<Self>(resp_text)?;
        Ok(statuses)
    }

    fn iter(&self) -> std::slice::Iter<TaskType> {
        self.0.iter()
    }

    /// Returns available task statuses for task.
    pub fn get_available_task_statuses(
        &self,
        task_type_name: &str,
    ) -> Vec<&str> {
        // TODO: change TaskTypes to HashMap
        // because we need to can find task_type.name without iterate.
        for task_type in self.iter() {
            if task_type.name == task_type_name {
                return task_type
                    .statuses
                    .iter()
                    .map(|status| status.name.as_str())
                    .collect();
            }
        }
        Vec::default()
    }
}

/// Struct for single task type.
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskType {
    #[serde(alias = "self")]
    link: String,
    id: String,
    name: String,
    subtask: bool,
    statuses: TaskStatuses,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatuses(Vec<TaskStatus>);

impl TaskStatuses {
    fn iter(&self) -> std::slice::Iter<TaskStatus> {
        self.0.iter()
    }
}

/// Struct for single task status.
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskStatus {
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    name: String,
    id: String,
    #[serde(alias = "statusCategory")]
    status_category: StatusCategory,
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

    #[test]
    fn test_deserialize_task_types() {
        let json_task_str = r#"
        [
            {
                "self": "https://link.com",
                "id": "10000",
                "name": "Task",
                "subtask": false,
                "statuses": [
                    {
                        "self": "https://link.com",
                        "description": "Open.",
                        "iconUrl": "https://link.com",
                        "name": "Open",
                        "id": "1",
                        "statusCategory": {
                            "self": "https://link.com",
                            "id": 2,
                            "key": "new",
                            "colorName": "blue-gray",
                            "name": "Open"
                        }
                    }
                ]
            },
            {
                "self": "https://link.com",
                "id": "10101",
                "name": "History",
                "subtask": false,
                "statuses": [
                    {
                        "self": "https://link.com",
                        "description": "Finished",
                        "iconUrl": "https://link.com",
                        "name": "DONE",
                        "id": "10104",
                        "statusCategory": {
                            "self": "https://link.com",
                            "id": 3,
                            "key": "done",
                            "colorName": "green",
                            "name": "Done"
                        }
                    }
                ]
            }
        ]
        "#;

        serde_json::from_str::<TaskTypes>(json_task_str).unwrap();
    }
}
