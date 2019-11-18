use std::error::Error;
use std::path::PathBuf;
use std::fs::read_to_string;
use structopt::StructOpt;
use regex::Regex;
use crate::model::*;
use crate::PROJECT_TEMP_FILE;

#[derive(StructOpt, Debug)]
pub struct CommitMsg {
    file: PathBuf,
}

pub fn commit_msg(args: CommitMsg) -> Result<(), Box<dyn Error>> {
    if !PathBuf::from(PROJECT_TEMP_FILE).exists() { return Ok(()); }
    let mut git_project = GitProject::open()?;

    let command_format = Regex::new(r"\[([^:\[\]]+)(?:@([^:\[\]]+))?: *([^:\[\]]+)\]")?;
    let message = read_to_string(&args.file)?;

    for command in command_format.captures_iter(&message) {
        let command_str = command.get(0).unwrap().as_str();
        let task_id: Id = if let Some(task_id) = command.get(1) { task_id.as_str().into() } else { continue }; // shouldn't reach the continue here, as pattern should not match
        let column_id: Id = if let Some(column_id) = command.get(3) { column_id.as_str().into() } else { continue }; // shouldn't reach the continue here either
        let project: &mut Project = match command.get(2).map(|m| m.as_str()) {
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
                        project.tasks().iter().find(|task| task.id() == &task_id).is_some()
                        && project.columns().iter().find(|column| column.id() == &column_id).is_some()
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
                    eprintln!("             Specify a project ID to disambiguate, e.g. [{}@{}: {}]", task_id, matching_projects.first().unwrap().id(), column_id);
                    std::process::exit(1);
                }
                matching_projects.into_iter().next().unwrap()
            }
        };

        if project.columns().iter().find(|column| column.id() == &column_id).is_none() {
            eprintln!("git-project: No column was found with ID {} in project {}, referenced in command {}", column_id, project.id(), command_str);
            std::process::exit(1);
        }
        if project.tasks().iter().find(|task| task.id() == &task_id).is_none() {
            eprintln!("git-project: No task was found with ID {} in project {}, referenced in command {}", task_id, project.id(), command_str);
            std::process::exit(1);
        }
        project.move_task_to_column(task_id.into(), column_id.into());
    }

    git_project.save()
}
