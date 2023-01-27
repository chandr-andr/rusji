use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

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
        })
    }
}

impl JiraTask {
    /// Creates new instance of JiraTask.
    ///
    /// Makes request
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

/// Struct for task types.
#[derive(Serialize, Debug)]
pub struct TaskTypes {
    types: Vec<TaskType>,
}

/// Struct for single task type.
#[derive(Serialize, Debug)]
struct TaskType {
    #[serde(alias = "self")]
    link: String,
    id: String,
    name: String,
    subtask: bool,
    statuses: Vec<TaskStatus>,
}

/// Struct for single task status.
#[derive(Serialize, Debug)]
struct TaskStatus {
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    name: String,
    id: String,
    category: StatusCategory,
}

/// Struct for single task category.
#[derive(Serialize, Debug)]
struct StatusCategory {
    #[serde(alias = "self")]
    link: String,
    id: String,
    key: String,
    name: String,
}

impl TaskTypes {
    /// Returns hashmap with keys task type name and values hashset with statuses ids.
    fn task_type_name_and_status_ids(&self) -> HashMap<&str, HashSet<&str>> {
        let mut type_name_status_ids: HashMap<&str, HashSet<&str>> = HashMap::new();

        for task_type in &self.types {
            let mut status_ids: HashSet<&str> = HashSet::new();

            for task_status in &task_type.statuses {
                status_ids.insert(&task_status.id);
            }

            type_name_status_ids.insert(&task_type.name, status_ids);
        }

        type_name_status_ids
    }
}
