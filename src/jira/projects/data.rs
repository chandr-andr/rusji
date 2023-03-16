use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::{errors::RusjiResult, jira::tasks::data::JiraTask, request_client::RequestClient};

#[derive(Serialize, Deserialize, Debug)]
pub struct JiraProjects(Vec<JiraProject>);

impl IntoIterator for JiraProjects {
    type Item = JiraProject;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let Self(projects) = self;

        projects.into_iter()
    }
}

impl JiraProjects {
    pub fn new(request_client: Arc<RwLock<RequestClient>>) -> RusjiResult<Self> {
        let response = request_client.read().unwrap().get_jira_projects()?;
        let resp_text = response.get_body();

        let projects = serde_json::from_str::<JiraProjects>(resp_text)?;
        Ok(projects)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
        let a = self.tasks.as_ref().unwrap().get(task_name).unwrap();
        a
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
