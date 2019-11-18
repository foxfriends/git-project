use std::error::Error;
use structopt::StructOpt;

mod pre_commit;
mod commit_msg;
mod prepare_commit_msg;
mod post_commit;

pub use pre_commit::*;
pub use commit_msg::*;
pub use prepare_commit_msg::*;
pub use post_commit::*;

#[derive(StructOpt, Debug)]
pub enum Hook {
    /// The pre-commit hook. Call this during the pre-commit Git hook, along with the
    /// matching post-commit to enable automatically committing the changes to .gitproject
    PreCommit,
    /// The prepare-commit-msg hook. Call this during the prepare-commit-msg Git hook to have
    /// the commit message text pre-filled with git-project commit command stubs.
    PrepareCommitMsg(PrepareCommitMsg),
    /// The commit-msg hook. Call this during the commit-msg Git hook to enable automatically
    /// updating task status based on a properly formatted commit message.
    CommitMsg(CommitMsg),
    /// The post-commit hook. Call this during the post-commit Git hook, along with the
    /// matching pre-commit to enable automatically committing the changes to .gitproject
    PostCommit,
}

pub fn hook(args: Hook) -> Result<(), Box<dyn Error>> {
    match args {
        Hook::PreCommit => pre_commit(),
        Hook::CommitMsg(args) => commit_msg(args),
        Hook::PrepareCommitMsg(args) => prepare_commit_msg(args),
        Hook::PostCommit => post_commit(),
    }
}
