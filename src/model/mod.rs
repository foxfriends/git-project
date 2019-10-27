use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct TaskID(String);

impl TaskID {
    pub fn new<I: AsRef<str>>(id: I) -> Self { Self(id.as_ref().to_string()) }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitProject {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub columns: Vec<Column>,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub description: Option<String>,
    pub tasks: Vec<TaskID>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: TaskID,
    pub tags: Vec<String>,
    pub name: String,
    pub assignee: String,
    pub description: String,
}
