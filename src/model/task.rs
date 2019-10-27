use serde::{Serialize, Deserialize};
use super::TaskID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: TaskID,
    pub tags: Vec<String>,
    pub name: String,
    pub assignee: String,
    pub description: String,
}
