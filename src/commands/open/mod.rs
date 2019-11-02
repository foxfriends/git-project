use std::env::current_dir;
use std::error::Error;
use git2::Repository;
use crate::model::*;

mod state;
use state::State;

pub fn open() -> Result<(), Box<dyn Error>> {
    let repository = Repository::discover(current_dir()?)?;
    let config = repository.config()?.snapshot()?;
    let current_user = config.get_string("user.email")?;

    let git_project = GitProject::open()?;

    let state = State::new(git_project, current_user);
    state.run()
}
