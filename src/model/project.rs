use std::collections::BTreeSet;
use serde::{Serialize, Deserialize};
use super::{Column, Task, TaskID};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    name: String,
    description: String,
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

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn columns(&self) -> &[Column] {
        self.columns.as_slice()
    }

    pub fn tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }

    pub fn task_with_id(&self, task_id: &TaskID) -> Option<&Task> {
        self.tasks.iter()
            .find(|task| task.id() == task_id)
    }

    pub fn column_of_task(&self, task: &Task) -> Option<&Column> {
        self.columns.iter()
            .find(|column| column.tasks().contains(task.id()))
    }

    pub fn column_index_of_task(&self, task: &Task) -> Option<usize> {
        self.columns.iter()
            .position(|column| column.tasks().contains(task.id()))
    }

    pub fn all_assignees(&self) -> BTreeSet<&str> {
        self.tasks.iter()
            .filter_map(|task| task.assignee())
            .collect()
    }

    pub fn all_tags(&self) -> BTreeSet<&str> {
        self.tasks.iter()
            .flat_map(|task| task.tags())
            .map(|string| string.as_str())
            .collect()
    }

    pub fn add_task(&mut self, task: Task, column: usize) -> bool {
        if self.tasks.iter().find(|t| t.id() == task.id()).is_some() { return false }
        self.columns[column].add_task(&task);
        self.tasks.push(task);
        true
    }

    pub fn replace_task(&mut self, original_task: &TaskID, task: Task, column: usize) {
        for column in self.columns.iter_mut() {
            column.remove_task(original_task);
        }
        self.tasks.retain(|task| task.id() != original_task);
        self.columns[column].add_task(&task);
        self.tasks.push(task);
    }

    pub fn delete_task(&mut self, task_id: &TaskID) {
        for column in self.columns.iter_mut() {
            column.remove_task(task_id);
        }
        self.tasks.retain(|task| task.id() != task_id);
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
        match self {
            ProjectBuilder { name, description: Some(description), columns, tasks } => Ok(Project { name, description, columns, tasks }),
            _=> Err(self),
        }
    }
}
