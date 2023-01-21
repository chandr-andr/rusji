use crate::errors::RusjiError;
use reqwest::blocking::Client;
use url::Url;

/// Struct for request response.
///
/// Now contain only body.
pub struct RequestResponse {
    body: String,
}

impl RequestResponse {
    /// Returns body from response.
    pub fn get_body(&self) -> &str {
        &self.body
    }
}

/// Struct for request client
///
/// Can make request and return a response or an RusjiError.
pub(crate) struct RequestClient {
    client: Client,
    jira_url: Url,
    request_credentials: String,
}

impl RequestClient {
    /// Creates new instance of `RequestClient`
    pub fn new(request_credentials: String, jira_url: &str) -> Self {
        Self {
            client: Client::new(),
            jira_url: Url::parse(jira_url).unwrap(),
            request_credentials,
        }
    }

    /// Returns all Jira projects.
    pub fn get_jira_projects(&self) -> Result<RequestResponse, RusjiError> {
        self.make_request(self.jira_url.join("/rest/api/2/project").unwrap())
    }

    /// Returns all tasks from project.
    pub fn get_tasks_from_project(
        &self,
        project_name: &str,
    ) -> Result<RequestResponse, RusjiError> {
        let project_tasks_endpoint =
            format!("/rest/api/2/search?jql=project={project_name}&expand=renderedFields",);
        self.make_request(self.jira_url.join(&project_tasks_endpoint).unwrap())
    }

    /// Returns new task.
    pub fn get_task(&self, task_key: &str) -> Result<RequestResponse, RusjiError> {
        self.make_request(
            self.jira_url
                .join(&format!(
                    "/rest/api/2/issue/{}?expand=renderedFields",
                    task_key
                ))
                .unwrap(),
        )
    }

    /// Makes a request.
    ///
    /// Returns `RequestResponse` or `RusjiError`.
    fn make_request(&self, url: Url) -> Result<RequestResponse, RusjiError> {
        let response_text = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!("Basic {}", self.request_credentials),
            )
            .header("Content-Type", "application/json")
            .send()?
            .text()?;
        Ok(RequestResponse {
            body: response_text,
        })
    }
}
