use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::jira::tasks::data::JiraTask;

#[derive(Serialize, Deserialize, Debug)]
pub struct JiraProject {
    #[serde(alias = "self")]
    pub link: String,
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub tasks: Option<HashMap<String, JiraTask>>,
}

impl JiraProject {
    pub fn get_task(&self, task_name: &str) -> &JiraTask {
        self.tasks.as_ref().unwrap().get(task_name).unwrap()
    }

    pub fn get_mut_task(&mut self, task_name: &str) -> &mut JiraTask {
        self.tasks.as_mut().unwrap().get_mut(task_name).unwrap()
    }

    pub fn tasks_names(&self) -> Option<Vec<String>> {
        let mut tasks_names: Vec<String> = Vec::default();
        if let Some(tasks) = self.tasks.as_ref() {
            for task in tasks.values() {
                tasks_names.push(format!("{} -- {}", &task.key, &task.summary));
            }
            return Some(tasks_names);
        }
        None
    }
}
