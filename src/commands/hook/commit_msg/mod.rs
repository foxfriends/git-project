use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::model::*;
use crate::PROJECT_TEMP_FILE;

mod common;
mod change_column;
mod change_tags;

#[derive(StructOpt, Debug)]
pub struct CommitMsg {
    file: PathBuf,
}

pub fn commit_msg(args: CommitMsg) -> Result<(), Box<dyn Error>> {
    if !PathBuf::from(PROJECT_TEMP_FILE).exists() { return Ok(()); }
    let mut git_project = GitProject::open()?;
    let message = read_to_string(&args.file)?;

    change_column::change_column(&mut git_project, message.as_str())?;
    change_tags::change_tags(&mut git_project, message.as_str())?;

    git_project.save()
}
