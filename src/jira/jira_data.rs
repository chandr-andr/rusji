use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::errors::{RusjiError, RusjiResult};
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
    pub selected_task: String,
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
            selected_task: String::default(),
        }
    }

    /// Sets new selected project.
    pub fn set_selected_project(&mut self, selected_project: &str) {
        self.selected_project = selected_project.to_string();
    }

    /// Sets new selected task.
    pub fn set_selected_task(&mut self, selected_task: &str) {
        let selected_task_key: &str = selected_task.split('-').collect::<Vec<&str>>()[1];
        self.selected_task = selected_task_key.to_string()
    }

    pub fn get_project(&self, project_name: &str) -> &JiraProject {
        self.projects.as_ref().unwrap().get(project_name).unwrap()
    }

    pub fn get_selected_project(&self) -> &JiraProject {
        self.projects
            .as_ref()
            .unwrap()
            .get::<String>(&self.selected_project)
            .unwrap()
    }

    pub fn get_mut_project(&mut self, project_name: &str) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut(project_name)
            .unwrap()
    }

    pub fn get_mut_selected_project(&mut self) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut::<String>(&self.selected_project)
            .unwrap()
    }

    pub fn get_selected_project_key(&self) -> String {
        let project = self.get_selected_project();
        project.key.clone()
    }

    pub fn get_selected_task_key(&self) -> String {
        format!("{}-{}", self.get_selected_project().key, self.selected_task)
    }

    pub fn update_projects(&mut self, jira_projects: Result<JiraProjects, RusjiError>) {
        match jira_projects {
            Ok(projects) => {
                let projects_field = self.make_projects_field(projects);
                self.projects = Some(projects_field);
            }
            Err(_) => {
                self.projects = None;
            }
        }
    }

    pub fn get_projects_names(&self) -> Vec<&str> {
        match self.projects.as_ref() {
            Some(project) => {
                return project.values().map(|prj| prj.name.as_str()).collect();
            }
            None => Vec::default(),
        }
    }

    pub fn update_tasks(&mut self, jira_tasks: Result<JiraIssues, RusjiError>) {
        match jira_tasks {
            Ok(tasks) => {
                let tasks_field = self.make_tasks_field(tasks);
                let mut project = self.get_mut_selected_project();
                project.tasks = Some(tasks_field);
            }
            Err(_) => {
                let mut project = self.get_mut_selected_project();
                project.tasks = None;
            }
        }
    }

    /// Adds new task to project.
    pub fn add_new_task(&mut self, task: Result<JiraTask, RusjiError>) {
        if let Ok(task) = task {
            self.get_mut_selected_project()
                .tasks
                .as_mut()
                .unwrap()
                .insert(task.key.clone(), task);
        }
    }

    pub fn find_project_by_subname(&self, project_subname: &str) -> Vec<&str> {
        let mut fit_projects: Vec<&str> = Vec::new();
        for project in self.projects.as_ref().unwrap().values() {
            let project_name_copy = project.name.clone();
            let available_condition = project.name.contains(project_subname)
                || project
                    .name
                    .contains(project_subname.to_uppercase().as_str())
                || project
                    .name
                    .contains(project_subname.to_lowercase().as_str())
                || project_name_copy.to_lowercase().contains(project_subname)
                || project_name_copy.to_uppercase().contains(project_subname);
            if available_condition {
                fit_projects.push(&project.name);
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
        let selected_projects_key = self.selected_project.clone();
        match task_key.parse::<usize>() {
            Ok(_) => {
                selected_task_key = format!("{}-{}", selected_projects_key, task_key);
            }
            Err(_) => {
                selected_task_key = task_key.to_string();
            }
        };
        let task = JiraTask::new(self.client.clone(), selected_task_key.as_str())?;
        let return_data = (task.summary.clone(), task.description.clone());

        let mut project = self.get_mut_project(selected_projects_key.as_str());

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
