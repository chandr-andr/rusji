use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JiraUsers(Vec<JiraUser>);

impl IntoIterator for JiraUsers {
    type Item = JiraUser;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let Self(jira_users) = self;

        jira_users.into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JiraUser {
    #[serde(alias = "self")]
    pub link: String,
    pub key: String,
    pub name: String,
    #[serde(alias = "emailAddress")]
    pub email_address: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub active: bool,
    #[serde(alias = "timeZone")]
    pub time_zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
