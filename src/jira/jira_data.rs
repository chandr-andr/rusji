use serde_json::{self};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::errors::RusjiResult;
use crate::jira::{
    projects::data::JiraProject,
    tasks::data::{JiraIssues, JiraTask},
};
use crate::request_client::RequestClient;

use super::projects::data::JiraProjects;
use super::tasks::data::TaskTypes;

use rusty_pool::ThreadPool;

/// Struct with data about company jira.
pub struct JiraData {
    projects: Option<HashMap<String, JiraProject>>,
    task_types: Option<TaskTypes>,
    pub client: Arc<RwLock<RequestClient>>,
    pub thread_pool: ThreadPool,
    pub selected_project: String,
}

impl JiraData {
    pub fn new(jira_url: &str, request_credentials: &str) -> Self {
        Self {
            projects: None,
            task_types: None,
            client: Arc::new(RwLock::new(RequestClient::new(
                request_credentials.to_string(),
                jira_url,
            ))),
            thread_pool: ThreadPool::default(),
            selected_project: String::default(),
        }
    }

    pub fn get_project(&self, project_name: &str) -> &JiraProject {
        self.projects.as_ref().unwrap().get(project_name).unwrap()
    }

    pub fn get_mut_project(&mut self, project_name: &str) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut(project_name)
            .unwrap()
    }

    pub fn update_projects(&mut self) -> RusjiResult<()> {
        let projects = JiraProjects::new(self.client.clone())?;
        let projects_field = self.make_projects_field(projects);
        self.projects = Some(projects_field);
        Ok(())
    }

    pub fn update_return_projects(&mut self) -> RusjiResult<Vec<&str>> {
        self.update_projects()?;
        Ok(self.get_projects_names())
    }

    pub fn get_projects_names(&self) -> Vec<&str> {
        let mut to_return_projects_names = Vec::<&str>::new();
        for project in self.projects.as_ref().unwrap().values() {
            to_return_projects_names.push(project.name.as_str());
        }
        to_return_projects_names
    }

    pub fn update_tasks(&mut self, project_name: &str) -> RusjiResult<()> {
        let tasks = JiraIssues::new(self.client.clone(), project_name)?;
        let tasks_field = self.make_tasks_field(tasks);

        let mut project = self.get_mut_project(project_name);
        project.tasks = Some(tasks_field);

        Ok(())
    }

    pub fn update_return_tasks(&mut self, project_name: &str) -> Vec<String> {
        let _ = self.update_tasks(project_name);
        if let Some(tasks) = self.get_project(project_name).tasks_names() {
            return tasks;
        }
        Vec::<String>::default()
    }

    pub fn find_project_by_subname(&self, project_subname: &str) -> Vec<&str> {
        let mut fit_projects: Vec<&str> = Vec::new();
        for project_name in self.projects.as_ref().unwrap().keys() {
            let project_name_copy = project_name.clone();
            let available_condition = project_name.contains(project_subname)
                || project_name.contains(project_subname.to_uppercase().as_str())
                || project_name.contains(project_subname.to_lowercase().as_str())
                || project_name_copy.to_lowercase().contains(project_subname)
                || project_name_copy.to_uppercase().contains(project_subname);
            if available_condition {
                fit_projects.push(project_name);
            }
        }
        fit_projects
    }

    pub fn find_task_by_subname(
        &self,
        task_subname: &str,
        selected_project: &str,
    ) -> Vec<&JiraTask> {
        let mut fit_tasks: Vec<&JiraTask> = Vec::new();
        let project = self.get_project(selected_project);
        for (task_name, task) in project.tasks.as_ref().unwrap().iter() {
            let task_name_copy = task_name.clone();
            let available_condition = task_name.contains(task_subname)
                || task_name.contains(task_subname.to_uppercase().as_str())
                || task_name.contains(task_subname.to_lowercase().as_str())
                || task_name_copy.to_lowercase().contains(task_subname)
                || task_name_copy.to_uppercase().contains(task_subname);
            if available_condition {
                fit_tasks.push(task);
            }
        }
        fit_tasks
    }

    pub fn get_new_task(&mut self, task_key: &str) -> RusjiResult<(String, String)> {
        let selected_task_key;
        match task_key.parse::<usize>() {
            Ok(_) => {
                let selected_projects_key = &self.get_project(&self.selected_project).key;
                selected_task_key = format!("{}-{}", selected_projects_key, task_key);
            }
            Err(_) => {
                selected_task_key = task_key.to_string();
            }
        };
        let response = self.client.read().unwrap().get_task(&selected_task_key)?;
        let resp_text = response.get_body();
        let task = serde_json::from_str::<JiraTask>(resp_text)?;
        let return_data = (task.summary.clone(), task.description.clone());

        let mut project = self.get_mut_project(&self.selected_project.clone());

        match project.tasks.as_mut() {
            Some(tasks) => {
                tasks.insert(task.key.clone(), task);
            }
            None => {
                let mut new_tasks = HashMap::<String, JiraTask>::new();
                new_tasks.insert(task.key.clone(), task);
                project.tasks = Some(new_tasks);
            }
        }

        Ok(return_data)
    }

    fn make_projects_field(&self, projects: JiraProjects) -> HashMap<String, JiraProject> {
        let mut projects_hashmap: HashMap<String, JiraProject> = HashMap::default();
        for project in projects {
            projects_hashmap.insert(project.name.clone(), project);
        }
        projects_hashmap
    }

    fn make_tasks_field(&self, tasks: JiraIssues) -> HashMap<String, JiraTask> {
        let mut tasks_hashmap: HashMap<String, JiraTask> = HashMap::default();
        for task in tasks {
            tasks_hashmap.insert(task.key.clone(), task);
        }
        tasks_hashmap
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
            "link": "https://jira.zxz.su/rest/api/2/issue/299756",
            "key": "FRE-39",
            "fields": {
                "description": "test description",
                "summary": "test summary"
            },
            "renderedFields": {
                "description": "test"
            },
            "status": {
                "self": "https://jira.zxz.su/rest/api/2/status/10104",
                "description": "Задача завершена",
                "iconUrl": "https://jira.zxz.su/images/icons/status_generic.gif",
                "name": "DONE",
                "id": "10104",
                "statusCategory": {
                    "self": "https://jira.zxz.su/rest/api/2/statuscategory/3",
                    "id": 3,
                    "key": "done",
                    "colorName": "green",
                    "name": "Выполнено"
                }
            }
        }
        "#;

        serde_json::from_str::<JiraTask>(json_task_str).unwrap();
    }

    #[test]
    fn test_deserialize_projects() {
        let json_task_str = r#"
        [

        ]
        "#;

        serde_json::from_str::<JiraTask>(json_task_str).unwrap();
    }
}
