use std::error::Error;
use regex::Match;
use crate::model::*;

pub fn resolve_project<'a>(git_project: &'a mut GitProject, task_id: &Id, column_id: Option<&Id>, project_match: Option<Match>, command_str: &str) -> Result<&'a mut Project, Box<dyn Error>> {
    let project: &mut Project = match project_match.map(|m| m.as_str()) {
        Some(project_id) => {
            if let Some(project) = git_project.projects_mut().iter_mut().find(|project| project.id().as_ref() == project_id) {
                project
            } else {
                eprintln!("git-project: No project was found with ID {}, used in command {}", project_id, command_str);
                std::process::exit(1);
            }
        }
        None => {
            let matching_projects: Vec<&mut Project> = git_project
                .projects_mut()
                .iter_mut()
                .filter(|project|
                    project.tasks().iter().find(|task| task.id() == task_id).is_some()
                    && (column_id.is_none()
                        || project.columns().iter().find(|column| column.id() == column_id.unwrap()).is_some())
                )
                .collect();
            if matching_projects.is_empty() {
                eprintln!("git-project: No task was found with ID {} in any project, referenced in command {}", task_id, command_str);
                std::process::exit(1);
            }
            if matching_projects.len() > 1 {
                eprintln!("git-project: Ambiguous task with ID {} is present in these projects:", task_id);
                for project in matching_projects.iter() {
                    eprintln!("             *   {} ({})", project.name(), project.id());
                }
                eprintln!("             Referenced in command {}", command_str);
                eprintln!("             Specify a project ID to disambiguate, e.g. {}@{}", task_id, matching_projects.first().unwrap().id());
                std::process::exit(1);
            }
            matching_projects.into_iter().next().unwrap()
        }
    };
    Ok(project)
}
