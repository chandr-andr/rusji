use serde::{Deserialize, Deserializer, Serialize};

pub struct AvailableTaskTransitions {
    transitions: Vec<AvailableTaskTransition>,
}

#[derive(Serialize)]
struct AvailableTaskTransition {
    id: String,
    name: String,
    name_in_to: String,
    description: String,
    to_id: usize,
}

impl<'de> Deserialize<'de> for AvailableTaskTransition {
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

        let task = Task::deserialize(deserializer)?;

        Ok(JiraTask {
            id: task.id,
            link: task.link,
            key: task.key,
            description: task.rendered_fields.description,
            summary: task.fields.summary,
            status: task.fields.status,
        })
    }
}
