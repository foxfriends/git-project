use structopt::StructOpt;
use std::error::Error;

#[derive(StructOpt, Debug)]
pub struct PrepareCommitMsg {
    file: String,
    source: String,
    amending: Option<String>,
}

pub fn prepare_commit_msg(args: PrepareCommitMsg) -> Result<(), Box<dyn Error>> {
    Ok(())
}
