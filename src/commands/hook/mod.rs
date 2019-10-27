use std::error::Error;
use structopt::StructOpt;

mod commit_msg;
mod prepare_commit_msg;

pub use commit_msg::*;
pub use prepare_commit_msg::*;

#[derive(StructOpt, Debug)]
pub enum Hook {
    /// The commit-msg hook. Call this during the commit-msg Git hook to enable automatically
    /// updating task status based on a properly formatted commit message.
    CommitMsg,
    /// The prepare-commit-msg hook. Call this during the prepare-commit-msg Git hook to have
    /// the commit message text pre-filled with git-project commit command stubs.
    PrepareCommitMsg,
}

pub fn hook(args: Hook) -> Result<(), Box<dyn Error>> {
    match args {
        Hook::CommitMsg => commit_msg(),
        Hook::PrepareCommitMsg => prepare_commit_msg(),
    }
}
