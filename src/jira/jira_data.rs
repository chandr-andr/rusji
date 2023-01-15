use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{self};
use std::collections::HashMap;


use crate::request_client::RequestClient;
use crate::errors::RusjiResult;

pub(crate) struct CursiveJiraData {
    pub jira_data: JiraData,
    pub selected_project: String,
}

impl CursiveJiraData {
    pub fn new(jira_data: JiraData) -> Self {
        CursiveJiraData {
            jira_data,
            selected_project: String::default(),
        }
    }

    pub fn update_projects(&mut self) -> RusjiResult<()> {
        self.jira_data.update_projects()?;
        Ok(())
    }

    pub fn update_return_projects(&mut self) -> RusjiResult<Vec<&str>> {
        self.update_projects()?;
        Ok(self.jira_data.get_projects_names())
    }

    pub fn update_tasks(&mut self, project_name: &str) {
        self.jira_data
            .update_tasks(project_name)
            .unwrap();
    }

    pub fn update_return_tasks(&mut self, project_name: &str) -> Vec<String> {
        self.update_tasks(project_name);
        if let Some(tasks) = self.jira_data.get_tasks_names_by_project(project_name) {
            return tasks;
        }
        Vec::<String>::default()
    }
}

#[derive(Serialize, Debug)]
pub struct JiraTask {
    id: String,
    #[serde(alias = "self")]
    link: String,
    key: String,
    description: String,
    summary: String,
}

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
        })
    }
}

struct JiraTaskStatus {

}

/// JiraIssues holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
struct JiraIssues {
    issues: Vec<JiraTask>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JiraProject {
    #[serde(alias = "self")]
    link: String,
    id: String,
    key: String,
    name: String,
    #[serde(skip_serializing, skip_deserializing)]
    tasks: Option<HashMap<String, JiraTask>>,
}

impl JiraProject {
    fn tasks_names(&self) -> Option<Vec<String>> {
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

/// Struct with data about company jira.
pub struct JiraData {
    projects: Option<HashMap<String, JiraProject>>,
    client: RequestClient,
}

impl JiraData {
    pub fn new(jira_url: &str, request_credentials: &str) -> Self {
        JiraData {
            projects: None,
            client: RequestClient::new(request_credentials.to_string(), jira_url),
        }
    }

    pub fn update_projects(&mut self) -> RusjiResult<()> {
        let binding = self.client.get_jira_projects()?;
        let resp_text = binding.get_body();

        let projects = serde_json::from_str::<Vec<JiraProject>>(resp_text)?;
        let projects_field = self.make_projects_field(projects);

        self.projects = Some(projects_field);
        Ok(())
    }

    pub fn get_projects_names(&self) -> Vec<&str> {
        let mut to_return_projects_names = Vec::<&str>::new();
        for project in self.projects.as_ref().unwrap().values() {
            to_return_projects_names.push(project.name.as_str());
        }
        to_return_projects_names
    }

    pub fn update_tasks(&mut self, project_name: &str) -> RusjiResult<()> {
        let binding = self.client.get_tasks_from_project(project_name)?;
        let resp_text = binding.get_body();

        let tasks = serde_json::from_str::<JiraIssues>(resp_text)?.issues;
        let tasks_field = self.make_tasks_field(tasks);

        let mut project = self.get_mut_project(project_name);
        project.tasks = Some(tasks_field);

        Ok(())
    }

    pub fn get_tasks_names_by_project(&mut self, project_name: &str) -> Option<Vec<String>> {
        let project = self.get_project(project_name);
        project.tasks_names()
    }

    pub fn get_task_description(&self, project_name: &str, task_name: &str) -> (&str, &str) {
        let project = self.get_project(project_name);
        let task = project.tasks.as_ref().unwrap().get(task_name).unwrap();

        (&task.summary, &task.description)
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

    pub fn find_task_by_subname(&self, task_subname: &str, selected_project: &str) -> Vec<String> {
        let mut fit_tasks: Vec<String> = Vec::new();
        let project = self
            .projects
            .as_ref()
            .unwrap()
            .get(selected_project)
            .unwrap();
        for (task_name, task) in project.tasks.as_ref().unwrap().iter() {
            let task_name_copy = task_name.clone();
            let available_condition = task_name.contains(task_subname)
                || task_name.contains(task_subname.to_uppercase().as_str())
                || task_name.contains(task_subname.to_lowercase().as_str())
                || task_name_copy.to_lowercase().contains(task_subname)
                || task_name_copy.to_uppercase().contains(task_subname);
            if available_condition {
                fit_tasks.push(format!("{} -- {}", task_name, task.summary));
            }
        }
        fit_tasks
    }

    pub fn get_new_task(
        &mut self,
        task_key: &str,
        selected_project: &str,
    ) -> RusjiResult<(String, String)> {
        let mut selected_task_key: String = String::default();
        match task_key.parse::<usize>() {
            Ok(_) => {
                let selected_projects_key = &self
                    .projects
                    .as_ref()
                    .unwrap()
                    .get(selected_project)
                    .unwrap()
                    .key;

                selected_task_key = format!("{}-{}", selected_projects_key, task_key);
            }
            Err(_) => {
                selected_task_key = task_key.to_string();
            }
        }
        let binding = self.client.get_task(&selected_task_key)?;
        let resp_text = binding.get_body();
        let task = serde_json::from_str::<JiraTask>(resp_text)?;
        let return_data = (task.summary.clone(), task.description.clone());

        let mut project = self
            .projects
            .as_mut()
            .unwrap()
            .get_mut(selected_project)
            .unwrap();

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

    fn make_projects_field(&self, projects: Vec<JiraProject>) -> HashMap<String, JiraProject> {
        let mut projects_hashmap: HashMap<String, JiraProject> = HashMap::default();
        for project in projects {
            projects_hashmap.insert(project.name.clone(), project);
        }
        projects_hashmap
    }

    fn make_tasks_field(&self, tasks: Vec<JiraTask>) -> HashMap<String, JiraTask> {
        let mut tasks_hashmap: HashMap<String, JiraTask> = HashMap::default();
        for task in tasks {
            tasks_hashmap.insert(task.key.clone(), task);
        }
        tasks_hashmap
    }

    fn get_project(&self, project_name: &str) -> &JiraProject {
        self.projects.as_ref().unwrap().get(project_name).unwrap()
    }

    fn get_mut_project(&mut self, project_name: &str) -> &mut JiraProject {
        self.projects
            .as_mut()
            .unwrap()
            .get_mut(project_name)
            .unwrap()
    }

    // fn get_task(&self, project: &'a JiraProject, task_name: &str) -> &'a JiraTask {
    //     project.tasks.as_ref().unwrap().get(task_name).unwrap()
    // }
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
            }
        }
        "#;

        serde_json::from_str::<JiraTask>(json_task_str).unwrap();
    }
}
