use std::error::Error;
use regex::Regex;
use crate::model::*;
use super::common;

pub fn change_column(git_project: &mut GitProject, message: &str) -> Result<(), Box<dyn Error>> {
    let change_column_command_format = Regex::new(r"\[([^\[\]\s]+)(?:@([^\[\]\s]+))? is *([^\[\]\s]+)\]")?; // [my-task is done], [new-task@ios is in-progress]

    for command in change_column_command_format.captures_iter(message) {
        let command_str = command.get(0).unwrap().as_str();

        let task_id: Id = if let Some(task_id) = command.get(1) { task_id.as_str().into() } else { continue }; // shouldn't reach the continue here, as pattern should not match
        let column_id: Id = if let Some(column_id) = command.get(3) { column_id.as_str().into() } else { continue }; // shouldn't reach the continue here either
        let project = common::resolve_project(git_project, &task_id, Some(&column_id), command.get(2), command_str)?;
        
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

    Ok(())
}
