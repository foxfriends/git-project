use serde::{Serialize, Deserialize};
use super::TaskID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    id: TaskID,
    tags: Vec<String>,
    name: String,
    assignee: Option<String>,
    description: String,
}

impl Task {
    pub fn new<I: AsRef<str>>(id: I) -> TaskBuilder {
        TaskBuilder::new(id.as_ref().to_string().into())
    }

    pub fn id(&self) -> &TaskID {
        &self.id
    }

    pub fn tags(&self) -> &[String] {
        self.tags.as_slice()
    }

    pub fn name(&self) -> &str {
        self.name.trim()
    }

    pub fn assignee(&self) -> Option<&str> {
        self.assignee.as_ref().map(String::as_str)
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }

    pub fn short_description(&self) -> &str {
        self.description.split("\n").next().unwrap().trim()
    }
}

#[derive(Debug)]
pub struct TaskBuilder {
    id: TaskID,
    tags: Vec<String>,
    name: Option<String>,
    assignee: Option<String>,
    description: Option<String>,
}

impl TaskBuilder {
    fn new(id: TaskID) -> TaskBuilder {
        TaskBuilder {
            id,
            tags: vec![],
            name: None,
            assignee: None,
            description: None,
        }
    }

    pub fn name<I: AsRef<str>>(self, name: I) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
            ..self
        }
    }

    pub fn description<I: AsRef<str>>(self, description: I) -> Self {
        Self {
            description: Some(description.as_ref().to_string()),
            ..self
        }
    }

    pub fn tag<I: AsRef<str>>(mut self, tag: I) -> Self {
        self.tags.push(tag.as_ref().to_string());
        self
    }

    pub fn assignee<I: AsRef<str>>(self, assignee: I) -> Self {
        Self {
            assignee: Some(assignee.as_ref().to_string()),
            ..self
        }
    }

    pub fn build(self) -> Result<Task, Self> {
        match self {
            TaskBuilder { id, tags, name: Some(name), assignee, description: Some(description) } => Ok(Task {
                id,
                tags,
                name,
                assignee,
                description,
            }),
            _ => Err(self)
        } 
    }
}
