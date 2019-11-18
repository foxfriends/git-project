use std::error::Error;
use std::fs::File;
use crate::PROJECT_TEMP_FILE;

pub fn pre_commit() -> Result<(), Box<dyn Error>> {
    File::create(PROJECT_TEMP_FILE)?;
    Ok(())
}
