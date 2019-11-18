use serde::{Serialize, Deserialize};
use super::{Task, Id};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    id: Id,
    name: String,
    description: String,
    tasks: Vec<Id>,
}

impl Column {
    pub fn new<I: AsRef<str>>(id: I) -> ColumnBuilder {
        ColumnBuilder::new(id.as_ref().to_string().into())
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn tasks(&self) -> &[Id] {
        self.tasks.as_slice()
    }

    pub fn add_task_id(&mut self, task: Id) {
        self.tasks.push(task);
    }

    pub fn add_task(&mut self, task: &Task) {
        self.tasks.push(task.id().clone());
    }

    pub fn remove_task(&mut self, task: &Id) {
        self.tasks.retain(|id| id != task);
    }

    pub fn without_tasks(&self) -> Column {
        Column {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            tasks: vec![],
        }
    }
}

#[derive(Debug)]
pub struct ColumnBuilder {
    id: Id,
    name: Option<String>,
    description: Option<String>,
    tasks: Vec<Id>,
}

impl ColumnBuilder {
    fn new(id: Id) -> Self {
        Self {
            id,
            name: None,
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

    pub fn name<I: AsRef<str>>(self, name: I) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
            ..self
        }
    }

    pub fn add_task_id(mut self, task: &Id) -> Self {
        self.tasks.push(task.clone());
        self
    }

    pub fn add_task(mut self, task: &Task) -> Self {
        self.tasks.push(task.id().clone());
        self
    }

    pub fn build(self) -> Result<Column, ColumnBuilder> {
        match self {
            ColumnBuilder { id, name: Some(name), description: Some(description), tasks } => Ok(Column { id, name, description, tasks }),
            _ => Err(self),
        }
    }
}
