use serde::{Serialize, Deserialize};
use super::{Column, Task};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub columns: Vec<Column>,
    pub tasks: Vec<Task>,
}
