use std::collections::BTreeSet;
use serde::{Serialize, Deserialize};
use super::{Column, Task, Id};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    id: Id,
    name: String,
    description: String,
    columns: Vec<Column>,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new<I: AsRef<str>>(id: I) -> ProjectBuilder {
        ProjectBuilder::new(id.as_ref().to_string().into())
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

    pub fn columns(&self) -> &[Column] {
        self.columns.as_slice()
    }

    pub fn tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }

    pub fn task_with_id(&self, task_id: &Id) -> Option<&Task> {
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

    pub fn replace_task(&mut self, original_task: &Id, task: Task, column: Option<usize>) {
        if let Some(column_index) = column {
            for column in self.columns.iter_mut() {
                column.remove_task(original_task);
            }
            self.columns[column_index].add_task(&task);
        }
        self.tasks.retain(|task| task.id() != original_task);
        self.tasks.push(task);
    }

    pub fn delete_task(&mut self, task_id: &Id) {
        for column in self.columns.iter_mut() {
            column.remove_task(task_id);
        }
        self.tasks.retain(|task| task.id() != task_id);
    }

    pub fn move_task(&mut self, task: &Task, distance: isize) {
        let previous_column = match self.column_index_of_task(task) {
            Some(column) => column,
            None => return,
        };
        let new_column = previous_column as isize + distance;
        if new_column < 0 || new_column > self.columns.len() as isize { return; }
        let new_column = new_column as usize;
        self.columns[previous_column].remove_task(task.id());
        self.columns[new_column].add_task(task);
    }

    pub fn move_task_to_column(&mut self, task_id: Id, column_id: Id) {
        for column in self.columns.iter_mut() {
            column.remove_task(&task_id);
        }
        let target_column = self.columns
            .iter_mut()
            .find(|column| column.id() == &column_id);
        if let Some(column) = target_column {
            column.add_task_id(task_id);
        }
    }
}

#[derive(Debug)]
pub struct ProjectBuilder {
    id: Id,
    name: Option<String>,
    description: Option<String>,
    columns: Vec<Column>,
    tasks: Vec<Task>,
}

impl ProjectBuilder {
    fn new(id: Id) -> Self {
        Self {
            id,
            name: None,
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

    pub fn name<I: AsRef<str>>(self, name: I) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
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
            ProjectBuilder { id, name: Some(name), description: Some(description), columns, tasks } => Ok(Project { id, name, description, columns, tasks }),
            _=> Err(self),
        }
    }
}
