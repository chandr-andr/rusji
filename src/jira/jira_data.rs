use std::{io::{Result as ioResult, Error, ErrorKind}, collections::HashMap};
use serde::{Deserialize, Serialize, Deserializer};
use reqwest::blocking::{Client, Response};
use serde_json::{self};
use url::Url;

pub(crate) struct CursiveJiraData<'a> {
    pub jira_data: JiraData<'a>,
    pub company_name: String,
    pub selected_project: String,
    pub encoded_creds: String,
}

impl<'a> CursiveJiraData<'a> {
    pub fn new(encoded_creds: String, company_name: &str, jira_data: JiraData<'a>) -> Self {
        CursiveJiraData {
            encoded_creds: encoded_creds,
            jira_data: jira_data,
            company_name: company_name.to_string(),
            selected_project: String::default(),
        }
    }

    pub fn update_projects(&mut self) {
        self.jira_data.update_projects(&self.encoded_creds).unwrap();
    }

    pub fn update_return_projects(&mut self) -> Vec<&str> {
        self.update_projects();
        self.jira_data.get_projects_names()
    }

    pub fn update_tasks(&mut self, project_name: &str) {
        self.jira_data.update_tasks(project_name, &self.encoded_creds).unwrap();
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
        where D: Deserializer<'de>
    {
        #[derive(Serialize, Deserialize, Debug)]
        struct Task {
            id: String,
            #[serde(alias = "self")]
            link: String,
            key: String,
            fields: Fields,
            #[serde(default = "default_rendered_fields", alias="renderedFields")]
            rendered_fields: RenderedFields
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
                description: "No description".to_string()
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

/// JiraIssues holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
struct JiraIssues {
    issues: Vec<JiraTask>
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
                tasks_names.push(
                    format!(
                        "{} -- {}", &task.key, &task.summary));
            }
            return Some(tasks_names)
        }
        None
    }
}

/// Struct with data about company jira.
pub struct JiraData<'a>{
    projects: Option<HashMap<String, JiraProject>>,
    jira_url: Url,
    client: Client,
    get_projects_url: &'a str,
    get_project_tasks_url: &'a str,
}

impl<'a> JiraData<'a> {
    pub fn new(jira_url: &str) -> Self {
        let jira_url = Url::parse(jira_url).unwrap();
        let client = Client::new();
        JiraData {
            projects: None,
            jira_url: jira_url,
            client: client,
            get_projects_url: "/rest/api/2/project",
            get_project_tasks_url: "/rest/api/2/search?jql=project=PRJ&expand=renderedFields",
        }
    }

    pub fn update_projects(&mut self, encoded_creds: &str) -> ioResult<()> {
        let url = self.jira_url.join(self.get_projects_url).unwrap();
        let response = self.make_get_request(url, encoded_creds)?;
        let resp_text = response.text().unwrap();

        let projects = serde_json::from_str::<Vec<JiraProject>>(
            resp_text.as_str(),
        )?;
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

    pub fn update_tasks(&mut self, project_name: &str, encoded_creds: &str) -> ioResult<()> {
        let url = self.jira_url.join(
            &self.get_project_tasks_url.replace("PRJ", project_name),
        ).unwrap();
        let response = self.make_get_request(url, encoded_creds)?;
        let resp_text = response.text().unwrap();

        let tasks = serde_json::from_str::<JiraIssues>(resp_text.as_str())?.issues;
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
        let task = self.get_task(project, task_name);

        (&task.summary, &task.description)
    }

    pub fn find_project_by_subname(&self, project_subname: &str) -> Vec<&str> {
        let mut fit_projects: Vec<&str> = Vec::new();
        for project_name in self.projects.as_ref().unwrap().keys() {
            let project_name_copy = project_name.clone();
            let available_condition =
                project_name.contains(project_subname)
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

    fn make_get_request(&self, url: Url, encoded_creds: &str) -> ioResult<Response> {
        let response = self.client
            .get(url)
            .header("Authorization", format!("Basic {encoded_creds}"))
            .header("Content-Type", "application/json")
            .send();
        match response {
            Ok(response) => Ok(response),
            Err(err) => Err(
                Error::new(
                    ErrorKind::Other,
                    err.to_string(),
                )
            )
        }
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
        self.projects.as_mut().unwrap().get_mut(project_name).unwrap()
    }

    fn get_task(&self, project: &'a JiraProject, task_name: &str) -> &'a JiraTask {
        project.tasks.as_ref().unwrap().get(task_name).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_task() {
        let json_task_str = r#"
        {
            "id": "299756",
            "link": "https://jira.zxz.su/rest/api/2/issue/299756",
            "key": "FRE-39",
            "fields": {
                "description": "test description",
                "summary": "test summary",
                "renderedFields": {
                    "description": "test"
                }
            }
        }
        "#;

        serde_json::from_str::<JiraTask>(json_task_str).unwrap();
    }
}