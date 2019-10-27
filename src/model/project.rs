use serde::{Serialize, Deserialize};
use super::{Column, Task};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    name: String,
    description: Option<String>,
    columns: Vec<Column>,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new<I: AsRef<str>>(name: I) -> ProjectBuilder {
        ProjectBuilder::new(name.as_ref().to_string())
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn columns(&self) -> &[Column] {
        self.columns.as_slice()
    }

    pub fn tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }
}

#[derive(Debug)]
pub struct ProjectBuilder {
    name: String,
    description: Option<String>,
    columns: Vec<Column>,
    tasks: Vec<Task>,
}

impl ProjectBuilder {
    fn new(name: String) -> Self {
        Self { 
            name,
            description: None,
            columns: vec![],
            tasks: vec![],
        }
    }

    pub fn description<I: AsRef<str>>(self, description: I) -> Self {
        Self {
            description: Some(description.as_ref().to_string()),
            ..self
        }
    }

    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn task(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn build(self) -> Result<Project, Self> {
        Ok(Project {
            name: self.name,
            description: self.description,
            columns: self.columns,
            tasks: self.tasks,
        })
    }
}
