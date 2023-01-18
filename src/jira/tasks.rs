use serde::Serialize;

#[derive(Serialize, Debug)]
struct TaskTypes {
    types: Vec<TaskType>
}

#[derive(Serialize, Debug)]
struct TaskType {
    #[serde(alias = "self")]
    link: String,
    id: usize,
    name: String,
    subtask: bool,
    statuses: Vec<TaskStatus>,
}

#[derive(Serialize, Debug)]
struct TaskStatus {
    #[serde(alias = "self")]
    link: String,
    description: String,
    #[serde(alias = "iconUrl")]
    icon_url: String,
    name: String,
    id: usize,
    category: StatusCategory,
}

#[derive(Serialize, Debug)]
struct StatusCategory {
    #[serde(alias = "self")]
    link: String,
    id: usize,
    key: String,
    name: String,
}
