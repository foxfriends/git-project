use structopt::StructOpt;

mod commands;
use commands::*;

const PROJECT_FILE_NAME: &'static str = ".gitproject";

/// Git-based project boards.
#[derive(Debug, StructOpt)]
enum Args {
    /// Opens the project board UI
    Open,
    /// Creates a new project, or sets up your repo to support git projects
    Init(Init),
    /// Check the tasks currently assigned to you
    Current,
    /// Adds the hooks to your repository. 
    ///
    /// The git-project hooks will be appended to the end of your existing Git hooks, if any.
    /// If you did not have existing Git hooks, this will be done automatically on running 
    /// `git project init`, so you should not need to use this command. 
    /// 
    /// If you wish to customize this behaviour, to better integrate with your workflow or 
    /// existing git hooks, feel free to add them manually.
    ///
    /// Note that hooks are *not* included when distributing your repository, so subsequent clones
    /// will require you to install hooks again.
    Hooks,
}

#[paw::main]
fn main(args: Args) {
    let result = match args {
        Args::Open => open(),
        Args::Init(args) => init(args),
        Args::Current => current(),
        Args::Hooks => hooks(),
    };

    if let Err(error) = result {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
