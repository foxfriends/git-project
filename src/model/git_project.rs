use serde::{Serialize, Deserialize};
use super::Project;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitProject {
    projects: Vec<Project>,
}

impl GitProject {
    pub fn new() -> GitProjectBuilder {
        GitProjectBuilder::new()
    }

    pub fn projects(&self) -> &[Project] {
        self.projects.as_slice()
    }
}

#[derive(Debug)]
pub struct GitProjectBuilder {
    projects: Vec<Project>,
}

impl GitProjectBuilder {
    fn new() -> Self {
        Self { projects: vec![] }
    }

    pub fn project(mut self, project: Project) -> Self {
        self.projects.push(project);
        self
    }

    pub fn build(self) -> Result<GitProject, Self> {
        Ok(GitProject { 
            projects: self.projects
        })
    }
}
