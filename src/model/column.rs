use serde::{Serialize, Deserialize};
use super::TaskID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub description: Option<String>,
    pub tasks: Vec<TaskID>,
}

