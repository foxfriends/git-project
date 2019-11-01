use serde::{Serialize, Deserialize};
use super::{Task, TaskID};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    name: String,
    description: Option<String>,
    tasks: Vec<TaskID>,
}

impl Column {
    pub fn new<I: AsRef<str>>(name: I) -> ColumnBuilder {
        ColumnBuilder::new(name.as_ref().to_string())
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn tasks(&self) -> &[TaskID] {
        self.tasks.as_slice()
    }

    pub fn add_task(&mut self, task: &Task) {
        self.tasks.push(task.id().clone());
    }
}

#[derive(Debug)]
pub struct ColumnBuilder {
    name: String,
    description: Option<String>,
    tasks: Vec<TaskID>,
}

impl ColumnBuilder {
    fn new(name: String) -> Self {
        Self { 
            name,
            description: None,
            tasks: vec![],
        }
    }

    pub fn description<I: AsRef<str>>(self, description: I) -> Self {
        Self {
            description: Some(description.as_ref().to_string()),
            ..self
        }
    }

    pub fn add_task(mut self, task: &Task) -> Self {
        self.tasks.push(task.id().clone());
        self
    }

    pub fn build(self) -> Result<Column, ColumnBuilder> {
        Ok(Column {
            name: self.name,
            description: self.description,
            tasks: self.tasks,
        })
    }
}
