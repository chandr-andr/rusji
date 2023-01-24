use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Deserializer, Serialize};

/// JiraIssues holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraIssues {
    pub issues: Vec<JiraTask>,
}

/// Struct for single task in Jira.
#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct JiraTaskStatus {
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    name: String,
    id: String,
}

/// Struct for task types.
#[derive(Serialize, Debug)]
struct TaskTypes {
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
