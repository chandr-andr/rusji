use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::io::{Result};
use crate::cli::RegisterJira;
use base64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    jira: Jira,
}

/// Base struct for Jira.
/// It has `companies` field.
#[derive(Serialize, Deserialize, Debug)]
pub struct Jira {
    pub companies: Vec<JiraCompany>,
}

/// Struct for Jira company.
/// Contains basic information about Jira company
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraCompany {
    pub link: String,
    pub company_name: String,
    pub encoded_credentials: String,
    company_projects: Option<Vec<CompanyProject>>,
}

/// Struct for Jira project.
#[derive(Serialize, Deserialize, Debug)]
pub struct CompanyProject {
    pub full_project_name: String,
    pub short_project_name: String,
    project_tasks: Vec<ProjectTask>,
}

/// Struct for project tasks.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectTask {
    pub number: String,
    pub description: String,
    pub link: Option<String>,
}

impl Company {
    pub fn new(json_string: &str) -> Result<Jira> {
        let company: Self = serde_json::from_str(json_string)?;
        Ok(company.jira)
    }
}

impl Jira {
    pub fn new(json_cfg: Value) -> Self {
        todo!()
    }
}

impl JiraCompany {
    fn from_cli(reg_jira: RegisterJira) -> Self {
        JiraCompany{
            link: reg_jira.link,
            company_name: reg_jira.company_name,
            encoded_credentials: base64::encode(
                format!("{}:{}", reg_jira.username, reg_jira.password),
            ),
            company_projects: None,
        }
    }
}

impl CompanyProject {
    fn new(json_cfg: Value) -> Self {
        todo!()
    }
}

impl ProjectTask {
    fn new(json_cfg: Value) -> Self {
        todo!();
    }
}