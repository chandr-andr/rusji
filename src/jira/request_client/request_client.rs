use crate::errors::RusjiError;
use reqwest::blocking::{Client, RequestBuilder};
use url::Url;

use super::request_models::IssueTransitionsReqData;

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
pub struct RequestClient {
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
        self.make_basic_request(
            self.jira_url.join("/rest/api/2/project").unwrap(),
        )
    }

    /// Returns all tasks from project.
    pub fn get_tasks_from_project(
        &self,
        project_name: &str,
    ) -> Result<RequestResponse, RusjiError> {
        let project_tasks_endpoint =
            format!("/rest/api/2/search?jql=project={project_name}&expand=renderedFields",);
        self.make_basic_request(
            self.jira_url.join(&project_tasks_endpoint).unwrap(),
        )
    }

    /// Returns new task.
    pub fn get_task(
        &self,
        task_key: &str,
    ) -> Result<RequestResponse, RusjiError> {
        self.make_basic_request(
            self.jira_url
                .join(&format!(
                    "/rest/api/2/issue/{}?expand=renderedFields",
                    task_key
                ))
                .unwrap(),
        )
    }

    /// Returns all available task statuses for project.
    pub fn get_task_statuses(
        &self,
        project_name: &str,
    ) -> Result<RequestResponse, RusjiError> {
        self.make_basic_request(
            self.jira_url
                .join(&format!("rest/api/2/project/{}/statuses", project_name))
                .unwrap(),
        )
    }

    /// Returns all available issue transitions for the task.
    pub fn get_issue_transitions(
        &self,
        issue_key: &str,
    ) -> Result<RequestResponse, RusjiError> {
        self.make_basic_request(
            self.jira_url
                .join(&format!("rest/api/2/issue/{}/transitions", issue_key))
                .unwrap(),
        )
    }

    /// Return all users with passed username.
    pub fn get_jira_users(
        &self,
        username: &str,
    ) -> Result<RequestResponse, RusjiError> {
        self.make_basic_request(
            self.jira_url
                .join(&format!("rest/api/2/user/search?username={}", username))
                .unwrap(),
        )
    }

    /// Updates task transition.
    pub fn update_task_transition(
        &self,
        issue_key: &str,
        transition_id: &str,
    ) -> Result<RequestResponse, RusjiError> {
        let mut request_data = IssueTransitionsReqData::new();
        request_data = request_data.add_transition_data(transition_id);

        let req_builder = self.post(
            self.jira_url
                .join(&format!("rest/api/2/issue/{}/transitions", issue_key))
                .unwrap(),
        );

        let response_text = req_builder
            .body(serde_json::to_string(&request_data)?)
            .send()?
            .text()?;

        Ok(RequestResponse {
            body: response_text,
        })
    }

    /// Makes a request.
    ///
    /// Returns `RequestResponse` or `RusjiError`.
    fn make_basic_request(
        &self,
        url: Url,
    ) -> Result<RequestResponse, RusjiError> {
        let response_text = self.get(url).send()?.text()?;
        Ok(RequestResponse {
            body: response_text,
        })
    }

    /// Adds basic fields to a request builder.
    ///
    /// It's necessary because in some cases we have additional
    /// parameters that should be added to the builder
    ///
    /// It is used only in methods like `get`, `post`, etc.
    fn builder_add_default_fields(
        &self,
        builder: RequestBuilder,
    ) -> RequestBuilder {
        builder
            .timeout(std::time::Duration::from_micros(5000000))
            .header(
                "Authorization",
                format!("Basic {}", self.request_credentials),
            )
            .header("Content-Type", "application/json")
    }

    /// Makes request builder for `get` request.
    fn get(&self, url: Url) -> RequestBuilder {
        let builder = self.client.get(url);
        self.builder_add_default_fields(builder)
    }

    /// Makes request builder for `post` request.
    fn post(&self, url: Url) -> RequestBuilder {
        let builder = self.client.post(url);
        self.builder_add_default_fields(builder)
    }
}
