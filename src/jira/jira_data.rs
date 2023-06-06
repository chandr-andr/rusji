use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::errors::RusjiError;
use crate::jira::{
    projects::data::JiraProject,
    tasks::data::{JiraIssue, JiraIssues},
};
use crate::request_client::request_client::RequestClient;

use super::projects::data::JiraProjects;

use rusty_pool::ThreadPool;

/// Struct with data about company jira.
pub struct JiraData {
    projects: Option<HashMap<String, JiraProject>>,
    pub client: Arc<RwLock<RequestClient>>,
    pub thread_pool: ThreadPool,
    pub selected_project: String,
    pub selected_task: String,
    pub activated_views: Vec<String>,
}

impl JiraData {
    pub fn new(jira_url: &str, request_credentials: &str) -> Self {
        Self {
            projects: None,
            client: Arc::new(RwLock::new(RequestClient::new(
                request_credentials.to_string(),
                jira_url,
            ))),
            thread_pool: ThreadPool::default(),
            selected_project: String::default(),
            selected_task: String::default(),
            activated_views: Vec::default(),
        }
    }

    /// Sets new selected project.
    pub fn set_selected_project(&mut self, selected_project: &str) {
        self.selected_project = selected_project.to_string();
    }

    /// Sets new selected task, in format `PRO-1`.
    pub fn set_selected_task(
        &mut self,
        raw_selected_task: &str,
    ) -> Option<&String> {
        let selected_task: String;
        let project_key = &self.get_selected_project()?.key;
        match raw_selected_task.parse::<usize>() {
            Ok(_) => {
                selected_task =
                    format!("{}-{}", project_key, raw_selected_task);
            }
            Err(_) => {
                let splited_task: Vec<&str> =
                    raw_selected_task.split(" -- ").collect();
                if splited_task.len() == 2 {
                    selected_task = splited_task[0].into();
                } else {
                    selected_task = raw_selected_task.to_string();
                }
            }
        };
        self.selected_task = selected_task;
        Some(&self.selected_task)
    }

    /// Returns immutable reference to a project.
    pub fn get_project(&self, project_name: &str) -> Option<&JiraProject> {
        if let Some(project) =
            self.projects.as_ref().unwrap().get(project_name)
        {
            return Some(project);
        };
        return None;
    }

    /// Returns mutable reference to a project.
    pub fn get_mut_project(&mut self, project_name: &str) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut(project_name)
            .unwrap()
    }

    /// Returns immutable reference to a selected project.
    pub fn get_selected_project(&self) -> Option<&JiraProject> {
        self.projects
            .as_ref()
            .unwrap()
            .get::<String>(&self.selected_project)
    }

    /// Returns mutable reference to a selected project.
    pub fn get_mut_selected_project(&mut self) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut::<String>(&self.selected_project)
            .unwrap()
    }

    /// Returns key of a selected project.
    ///
    /// Assume we have project with name `PROJECT`
    /// and the key `PRO`.
    /// This method will return `PRO`
    pub fn get_selected_project_key(&self) -> String {
        let project = self.get_selected_project();
        project.unwrap().key.clone()
    }

    /// Returns immutable reference to a task.
    pub fn get_selected_task(&self) -> &JiraIssue {
        self.get_selected_project()
            .unwrap()
            .get_task(&self.selected_task)
    }

    /// Returns mutable reference to a task.
    pub fn get_mut_selected_task(&mut self) -> &mut JiraIssue {
        let issue_key = self.get_selected_task().key.clone();
        self.get_mut_selected_project().get_mut_task(&issue_key)
    }

    /// Updates projects.
    ///
    /// This method receives `Result<JiraProjects, RusjiError>`
    ///
    /// If `jira_projects` is Ok updates `projects` field else pass `None`.
    pub fn update_projects(
        &mut self,
        jira_projects: Result<JiraProjects, RusjiError>,
    ) {
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

    /// Returns names of projects.
    pub fn get_projects_names(&self) -> Vec<&str> {
        match self.projects.as_ref() {
            Some(project) => {
                return project
                    .values()
                    .map(|prj| prj.name.as_str())
                    .collect();
            }
            None => Vec::default(),
        }
    }

    /// Updates tasks.
    ///
    /// This method receives `Result<JiraIssues, RusjiError>`
    ///
    /// If `jira_projects` is Ok updates `tasks` field else pass `None`.
    pub fn update_tasks(
        &mut self,
        jira_tasks: Result<JiraIssues, RusjiError>,
    ) {
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
    pub fn add_new_task(&mut self, task: JiraIssue) {
        self.get_mut_selected_project()
            .tasks
            .as_mut()
            .unwrap()
            .insert(task.key.clone(), task);
    }

    /// Tries to find project by subname.
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

    /// Tries to find task by subname
    pub fn find_task_by_subname(
        &self,
        task_subname: &str,
        selected_project: &str,
    ) -> Option<Vec<&JiraIssue>> {
        let mut fit_tasks: Vec<&JiraIssue> = Vec::new();
        let project = self.get_project(selected_project)?;

        for (task_name, task) in project.tasks.as_ref()?.iter() {
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
        Some(fit_tasks)
    }

    /// Builds `projects` field from `JiraProjects`.
    fn make_projects_field(
        &self,
        projects: JiraProjects,
    ) -> HashMap<String, JiraProject> {
        let mut projects_hashmap: HashMap<String, JiraProject> =
            HashMap::default();
        for project in projects {
            projects_hashmap.insert(project.name.clone(), project);
        }
        projects_hashmap
    }

    /// Builds `tasks` field from `JiraIssues`
    fn make_tasks_field(
        &self,
        tasks: JiraIssues,
    ) -> HashMap<String, JiraIssue> {
        let mut tasks_hashmap: HashMap<String, JiraIssue> = HashMap::default();
        for task in tasks {
            tasks_hashmap.insert(task.key.clone(), task);
        }
        tasks_hashmap
    }
}
