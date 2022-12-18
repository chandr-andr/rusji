use std::io::{Result, Error, ErrorKind};
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use serde_json;
use url::Url;


/// JiraTaskFields holds all necessary fields
/// to describe a task.
#[derive(Serialize, Deserialize, Debug)]
struct JiraTaskFields {
    description: String,
    summary: String,
}

/// JiraTask holds all necessary information
/// about task to interact with it.
#[derive(Serialize, Deserialize, Debug)]
struct JiraTask {
    id: usize,
    #[serde(alias = "self")]
    link: String,
    key: String,
    fields: JiraTaskFields,
}


#[derive(Serialize, Deserialize, Debug)]
struct JiraProject {
    #[serde(alias = "self")]
    link: String,
    id: String,
    key: String,
    name: String,
    #[serde(skip_serializing, skip_deserializing)]
    tasks: Vec<JiraTask>,
}

/// Struct with data about company jira.
pub struct JiraData<'a>{
    projects: Option<Vec<JiraProject>>,
    jira_url: Url,
    client: Client,
    get_projects_url: &'a str,
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
        }
    }

    pub fn update_projects(&mut self, encoded_creds: &str) -> Result<()> {
        let url = self.jira_url.join(self.get_projects_url).unwrap();
        let response = self.client
            .get(url)
            .header("Authorization", format!("Basic {encoded_creds}"))
            .header("Content-Type", "application/json")
            .send();
        match response {
            Ok(response) => {
                let resp_text = response.text().unwrap();
                let projects = serde_json::from_str::<Vec<JiraProject>>(
                    resp_text.as_str(),
                )?;
                self.projects = Some(projects);
                Ok(())
            },
            Err(err) => Err(
                Error::new(
                    ErrorKind::Other,
                    err.to_string(),
                )
            )
        }
    }

    pub fn get_projects_names(&self) -> Vec<&str> {
        let mut to_return_projects_names = Vec::<&str>::new();
        for project in self.projects.as_ref().unwrap() {
            to_return_projects_names.push(project.name.as_str());
        }
        to_return_projects_names
    }
}
