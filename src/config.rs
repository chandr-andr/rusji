use crate::constance::*;
use crate::utils::*;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};
extern crate base64;

/// Structure for main information about a jira company.
#[derive(Serialize, Deserialize, Debug)]
pub struct Jira {
    url: String,
    encoded_creds: String,
}

impl Jira {
    /// Creates new instance of Jira.
    /// #### Base usage:
    ///
    /// ```
    /// let jira = Jira::new(
    ///     "url_to_jira".to_string(),
    ///     "username".to_string(),
    ///     "password".to_string(),
    /// );
    /// ```
    fn new(url: String, username: String, password: String) -> Self {
        let encoded_creds =
            base64::encode(format!("{}:{}", username, password));
        Jira { url, encoded_creds }
    }

    pub fn get_url(&self) -> &str {
        self.url.as_str()
    }

    pub fn get_encoded_creds(&self) -> &str {
        self.encoded_creds.as_str()
    }
}

/// Structure for Jira company.
#[derive(Serialize, Deserialize, Debug)]
struct Company {
    company_name: String,
    jira: Jira,
}

impl Company {
    fn new(company_name: String, jira: Jira) -> Self {
        Company { company_name, jira }
    }
}

/// Main structure for app config.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    companies: Vec<Company>,
    #[serde(skip_serializing, skip_deserializing)]
    config_path: String,
}

impl Config {
    /// Creates new instance of Config.
    /// Basic usage:
    ///
    /// ```
    /// let config = Config::new();
    /// ```
    pub fn new() -> Result<Self> {
        let app_config = std::fs::read_to_string(build_app_config_path()?)?;
        let config_path = Self::get_config_path()?;
        match serde_json::from_str::<Config>(&app_config) {
            Ok(mut config) => {
                config.config_path = config_path;
                Ok(config)
            }
            Err(_) => Err(Error::new(
                ErrorKind::Other,
                "Can't read config! Is it correct?",
            )),
        }
    }

    /// Returns vector with names of the companies.
    pub fn companies_names(&self) -> Vec<String> {
        let mut companies_names = Vec::<String>::new();
        for company in &self.companies {
            companies_names.push(company.company_name.clone());
        }
        companies_names
    }

    /// Adds new company to the config.
    /// Creates new instance of `Jira` and `Company`
    /// structures.
    /// Then serialize our new config to json and write it in file.
    pub fn add_new_company(
        &mut self,
        url: &str,
        company_name: &str,
        username: &str,
        password: &str,
    ) -> Result<()> {
        let jira_data = Jira::new(
            url.to_string(),
            username.to_string(),
            password.to_string(),
        );
        let company_data = Company::new(company_name.to_string(), jira_data);
        self.companies.push(company_data);
        std::fs::write(&self.config_path, serde_json::to_string_pretty(&self)?)
    }

    /// Deletes exist company.
    pub fn delete_company(&mut self, company_name: &str) -> Result<()> {
        let mut to_remove_company_idx = usize::default();
        let mut idx = 0;
        for company in &self.companies {
            if company.company_name == company_name {
                to_remove_company_idx = idx;
            }
            idx += 1;
        }

        self.companies.remove(to_remove_company_idx);
        std::fs::write(&self.config_path, serde_json::to_string_pretty(&self)?)
    }

    pub fn get_jira_by_company(&self, company_name: &str) -> Result<&Jira> {
        for company in &self.companies {
            if company.company_name == company_name {
                return Ok(&company.jira);
            }
        }
        Err(Error::new(ErrorKind::Other, "Not found!"))
    }

    fn get_config_path() -> Result<String> {
        match build_full_app_path() {
            Ok(path) => Ok(format!("{}/{}", path, APP_CONFIG,)),
            Err(err) => Err(err),
        }
    }
}

pub fn build_full_app_path() -> Result<String> {
    let home_dir = home_dir();
    match home_dir {
        Some(path) => Ok(format!("{}/{}", path.display(), APP_DIRECTORY,)),
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Can't find home directory!",
        )),
    }
}
