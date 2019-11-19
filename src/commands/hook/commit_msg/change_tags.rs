use std::error::Error;
use regex::Regex;
use crate::model::*;
use super::common;

pub fn change_tags(git_project: &mut GitProject, message: &str) -> Result<(), Box<dyn Error>> {
    let change_tags_command_format = Regex::new(r"\[([^\[\]\s]+)(?:@([^\[\]\s]+))?((?:[\s]+(?:[+-][\S]+))+)\]")?; // [my-task +bug], [my-task@ios -blocked +important]

    for command in change_tags_command_format.captures_iter(message) {
        let command_str = command.get(0).unwrap().as_str();
        let task_id: Id = if let Some(task_id) = command.get(1) { task_id.as_str().into() } else { continue }; // shouldn't reach the continue here, as pattern should not match
        let tags: &str = if let Some(tags) = command.get(3) { tags.as_str() } else { continue };
        let project = common::resolve_project(git_project, &task_id, None, command.get(2), command_str)?;

        let mut task = if let Some(task) = project.task_with_id(&task_id) { task.clone() } else {
            eprintln!("No task was found with ID {} in project {}, referenced in command {}", task_id, project.id(), command_str);
            std::process::exit(1);
        };

        for tag in tags.split(|ch: char| ch.is_whitespace()).filter(|s| !s.is_empty()) {
            eprintln!("match: {:?}", tag);
            match tag.split_at(1) {
                ("+", tag) => task.add_tag(tag),
                ("-", tag) => task.remove_tag(tag),
                _ => (),
            }
        }

        project.replace_task(&task_id, task, None);
    }

    Ok(())
}
