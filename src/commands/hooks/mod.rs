use std::env::current_dir;
use std::error::Error;
use std::fs::{OpenOptions, read_to_string};
use std::path::PathBuf;
use std::io::Write;
#[cfg(unix)] use std::fs::{set_permissions, metadata};
#[cfg(unix)] use std::os::unix::fs::PermissionsExt;
use regex::Regex;
use git2::Repository;

fn hook(path: PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        let pattern = Regex::new(&format!("^[^#]*git project hook {}", name)).unwrap();
        let contents = read_to_string(&path)?;
        for line in contents.lines() {
            if pattern.is_match(line) { return Ok(()); }
        }
    }
    let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
    writeln!(file)?;
    writeln!(file, "# git-project {} hook", name)?;
    writeln!(file, "git project hook {} $@", name)?;
    std::mem::drop(file);
    #[cfg(unix)] {
        let mut permissions = metadata(&path)?.permissions();
        permissions.set_mode(0o777);
        set_permissions(&path, permissions)?;
    }

    Ok(())
}

pub fn hooks() -> Result<(), Box<dyn Error>> {
    let repository = Repository::discover(current_dir()?)?;
    let hooks = repository.path().join("hooks");

    hook(hooks.join("pre-commit"), "pre-commit")?;
    hook(hooks.join("post-commit"), "post-commit")?;
    hook(hooks.join("commit-msg"), "commit-msg")?;
    hook(hooks.join("prepare-commit-msg"), "prepare-commit-msg")?;

    Ok(())
}
