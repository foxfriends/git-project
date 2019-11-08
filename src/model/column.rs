use serde::{Serialize, Deserialize};
use super::{Task, TaskID};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    name: String,
    description: String,
    tasks: Vec<TaskID>,
}

impl Column {
    pub fn new<I: AsRef<str>>(name: I) -> ColumnBuilder {
        ColumnBuilder::new(name.as_ref().to_string())
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn tasks(&self) -> &[TaskID] {
        self.tasks.as_slice()
    }

    pub fn add_task(&mut self, task: &Task) {
        self.tasks.push(task.id().clone());
    }

    pub fn remove_task(&mut self, task: &TaskID) {
        self.tasks.retain(|id| id != task);
    }

    pub fn without_tasks(&self) -> Column {
        Column {
            name: self.name.clone(),
            description: self.description.clone(),
            tasks: vec![],
        }
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
        match self {
            ColumnBuilder { name, description: Some(description), tasks } => Ok(Column { name, description, tasks }),
            _ => Err(self),
        }
    }
}
