use std::io::{Result as ioResult, Error, ErrorKind};
use serde::{Deserialize, Serialize, Deserializer};
use reqwest::blocking::{Client, Response};
use serde_json::{self};
use url::Url;

#[derive(Serialize, Debug)]
struct JiraTask {
    id: String,
    #[serde(alias = "self")]
    link: String,
    key: String,
    description: Option<String>,
    summary: String,
}

impl<'de> Deserialize<'de> for JiraTask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Serialize, Deserialize, Debug)]
        struct Outer {
            id: String,
            #[serde(alias = "self")]
            link: String,
            key: String,
            fields: Inner,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Inner {
            description: Option<String>,
            summary: String,
        }


        let helper = Outer::deserialize(deserializer)?;
        Ok(JiraTask {
            id: helper.id,
            link: helper.link,
            key: helper.key,
            description: helper.fields.description,
            summary: helper.fields.summary,
        })
    }
}

/// JiraTask holds all necessary information
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
    tasks: Option<Vec<JiraTask>>,
}

impl JiraProject {

    fn tasks_names(&self) -> Option<Vec<String>> {
        let mut tasks_names: Vec<String> = Vec::default();
        if let Some(tasks) = self.tasks.as_ref() {
            for task in tasks {
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
    projects: Option<Vec<JiraProject>>,
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
            get_project_tasks_url: "/rest/api/2/search?jql=project=PRJ",
        }
    }

    pub fn update_projects(&mut self, encoded_creds: &str) -> ioResult<()> {
        let url = self.jira_url.join(self.get_projects_url).unwrap();
        let response = self.make_get_request(url, encoded_creds)?;
        let resp_text = response.text().unwrap();
        let projects = serde_json::from_str::<Vec<JiraProject>>(
            resp_text.as_str(),
        )?;
        self.projects = Some(projects);
        Ok(())
    }

    pub fn get_projects_names(&self) -> Vec<&str> {
        let mut to_return_projects_names = Vec::<&str>::new();
        for project in self.projects.as_ref().unwrap() {
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
        for project in self.projects.as_mut().unwrap() {
            if project.name == project_name {
                project.tasks = Some(tasks);
                return Ok(())
            }
        }
        Ok(())
    }

    pub fn get_tasks_names_by_project(&self, project_name: &str) -> Option<Vec<String>> {
        for project in self.projects.as_ref().unwrap() {
            if project.name == project_name {
                return project.tasks_names();
            }
        }
        None
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
}
