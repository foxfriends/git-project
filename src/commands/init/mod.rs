use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::fmt::{self, Display, Formatter};
use git2::Repository;
use structopt::StructOpt;
use crate::PROJECT_FILE_NAME;
use crate::model::*;

#[derive(Debug, StructOpt)]
pub struct Init {
    /// The name of your project. Defaults to the name of the project directory.
    #[structopt(long)]
    name: Option<String>,
    /// Do not automatically add the git-project hooks, even if you have no existing Git hooks.
    #[structopt(long)]
    no_hooks: bool,
    /// Add hooks as part of the init procedure, even if you already have existing hooks.
    /// Including this flag is the same as running `git project hooks` separately
    #[structopt(long)]
    force_hooks: bool,
}

#[derive(Debug)]
struct InitError;
impl Display for InitError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} file already exists. Did you already initialize a project in this repository?", PROJECT_FILE_NAME)
    }
}
impl Error for InitError {}

pub fn init(args: Init) -> Result<(), Box<dyn Error>> {
    let repository = Repository::discover(current_dir()?)?;
    let workdir = repository.workdir().unwrap();
    let root = workdir.join(PROJECT_FILE_NAME);
    if root.exists() { return Err(Box::new(InitError)) }

    let hooks_path = repository.path().join("hooks");
    let hooks_exist = hooks_path.exists() && (hooks_path.join("commit-msg").exists() || hooks_path.join("prepare-commit-msg").exists());
    let will_add_hooks = !args.no_hooks && (!hooks_exist || args.force_hooks);

    let hooks_message = if will_add_hooks {
        "1.  The git hooks have been set up automatically, so that part is already ready to go."
    } else {
        "1.  Git hooks have not been added to your repository yet. If automatic task management is a feature you are interested in, run `git project hooks` to append them to your existing hooks, or integrate them manually (see the project README online)."
    };

    let config = repository.config()?.snapshot()?;
    let assignee = config.get_string("user.email")?;
    let name = args.name
        .or_else(|| workdir.file_name().and_then(|osstr| osstr.to_str()).map(str::to_string))
        .unwrap_or("New project".to_string());

    let task = Task::new("git-project")
        .name("Welcome to Git project")
        .tag("meta")
        .assignee(assignee)
        .description(format!(r#"Your first task is to set up your project board. Give your project a name and description, make sure the columns are to your liking, and maybe even put in a few tasks!

Here's how:
{}
2.  Open the editor by running `git project open`, and set the name, description, columns, and maybe even add some tasks.
3.  There's no need to move this task to the "Done" column - it will be moved automatically later (assuming you set the hooks up)
4.  Commit your changes. There will be instructions in the generated commit message for how to proceed.
"#, hooks_message))
        .build().unwrap();

    let git_project = GitProject::new()
        .project(Project::new(name)
            .description("Write a description of your project here.")
            .column(Column::new("New")
                .description("Tasks that have not yet been started")
                .build().unwrap()
            )
            .column(Column::new("In Progress")
                .description("Tasks that are currently being worked on")
                .add_task(&task)
                .build().unwrap()
            )
            .column(Column::new("Done")
                .description("Tasks that have been completed recently")
                .build().unwrap()
            )
            .task(task)
            .build().unwrap()
        )
        .build().unwrap();

    let project_string = toml::to_string_pretty(&git_project)?;
    let mut file = File::create(root)?;
    write!(file, "{}", project_string)?;

    if will_add_hooks {
        super::hooks()?;
    }

    super::current()
}
