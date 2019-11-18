use std::error::Error;
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;
use crate::PROJECT_TEMP_FILE;
use crate::PROJECT_FILE_NAME;

// TODO: would be nice to use the git library for this, but it would be much more complicated
pub fn post_commit() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(PROJECT_TEMP_FILE);
    if path.exists() {
        remove_file(&path)?;
        Command::new("git").arg("add").arg(PROJECT_FILE_NAME).spawn()?;
        Command::new("git").arg("commit").arg("--amend").arg("--no-verify").arg("--quiet").spawn()?;
    }
    Ok(())
}
