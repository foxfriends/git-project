use serde::{Serialize, Deserialize};
use super::Project;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitProject {
    pub projects: Vec<Project>,
}
