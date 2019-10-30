use std::env::current_dir;
use std::error::Error;
use git2::Repository;
use crate::model::*;

pub fn current() -> Result<(), Box<dyn Error>> {
    let repository = Repository::discover(current_dir()?)?;
    let config = repository.config()?.snapshot()?;
    let current_user = config.get_str("user.email")?;

    let git_project = GitProject::open()?;
    for project in git_project.projects() {
        let my_tasks: Vec<&Task> = project
            .tasks()
            .iter()
            .filter(|task| task.assignee() == Some(current_user))
            .collect();

        if my_tasks.is_empty() { continue; }

        println!("Project: \x1b[32m{}\x1b[0m. Tasks assigned: \x1b[33m{}\x1b[0m", project.name(), my_tasks.len());
        for task in &my_tasks {
            println!();
            let status = project.column_of_task(task).map(|col| col.name()).unwrap_or("\x1b[97mUnknown\x1b[0m");
            println!("\x1b[33m[{}]\x1b[0m  (Status: \x1b[35m{}\x1b[0m)", task.id(), status);
            for line in task.name().lines() {
                println!("\t\x1b[1m{}\x1b[0m", line);
            }
            println!();
            for line in task.description().lines() {
                println!("\t{}", line);
            }
        }
    }

    Ok(())
}
