use std::env::current_dir;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::read_to_string;
use git2::Repository;
use serde::{Serialize, Deserialize};
use super::Project;
use crate::PROJECT_FILE_NAME;

#[derive(Debug)]
struct OpenError;
impl Display for OpenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} file does not exist. Did you forget to initialize a project in this repository?", PROJECT_FILE_NAME)
    }
}
impl Error for OpenError {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitProject {
    projects: Vec<Project>,
}

impl GitProject {
    pub fn new() -> GitProjectBuilder {
        GitProjectBuilder::new()
    }

    pub fn open() -> Result<GitProject, Box<dyn Error>> {
        let repository = Repository::discover(current_dir()?)?;
        let workdir = repository.workdir().unwrap();
        let root = workdir.join(PROJECT_FILE_NAME);
        if !root.exists() { return Err(Box::new(OpenError)) }
        let string = read_to_string(root)?;
        let git_project = toml::from_str(&string)?;

        Ok(git_project)
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
