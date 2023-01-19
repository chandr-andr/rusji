use std::collections::{HashMap, HashSet};

use serde::Serialize;

#[derive(Serialize, Debug)]
struct TaskTypes {
    types: Vec<TaskType>
}

#[derive(Serialize, Debug)]
struct TaskType {
    #[serde(alias = "self")]
    link: String,
    id: String,
    name: String,
    subtask: bool,
    statuses: Vec<TaskStatus>,
}

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