use std::path::Path;

use anyhow::Result;
use inquire::Confirm;
use tracing::instrument;

/// Creates a directory if it doesn't exist and the provided `dry_run` flag is not set.
#[macro_export]
macro_rules! create_dir_gracefully {
    ($dir:expr, $dry_run:expr) => {
        if !$dry_run {
            tracing::debug!("Creating directory {:?}", $dir);
            std::fs::create_dir_all($dir)?;
        }
    };
}

pub(crate) use create_dir_gracefully;

/// Checks if rust artifacts are present in the given directory.
/// If `dry_run` is enabled, this method will not error if rust
/// artifacts are found.
#[instrument(name = "utils", skip(dir, ci, dry_run))]
pub(crate) fn check_artifacts(dir: &Path, ci: bool, dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    let mut prompted = false;
    if dir.join("Cargo.toml").exists() {
        tracing::warn!("Rust artifacts detected in the project directory");
        if !Confirm::new("[WARNING] Found conflicting files. Are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            anyhow::bail!("User aborted after detecting rust artifacts in the project directory");
        }
        prompted = true;
    }
    if !prompted && dir.join("LICENSE").exists() {
        tracing::warn!("LICENSE detected in the project directory");
        if !Confirm::new("[WARNING] Found conflicting files. Are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            anyhow::bail!("User aborted after detecting existing license in the project directory");
        }
        prompted = true;
    }
    if !prompted && dir.join("README.md").exists() {
        tracing::warn!("README detected in the project directory");
        if !Confirm::new("[WARNING] Found README.md in the project directory. Proceeding will overwrite this file. Are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            anyhow::bail!("User aborted after detecting existing readme in the project directory");
        }
        prompted = true;
    }
    if !prompted
        && ci
        && dir
            .join(".github")
            .join("workflows")
            .join("ci.yml")
            .exists()
    {
        tracing::warn!("Rust artifacts detected in the project directory");
        if !Confirm::new("[WARNING] Found conflicting files. Are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            anyhow::bail!("User aborted after detecting rust artifacts in the project directory");
        }
    }
    Ok(())
}
