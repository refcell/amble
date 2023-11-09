use anyhow::Result;
use std::path::Path;
use tracing::instrument;

/// Create git repository with given github username. If github username is not specified, try to grab one
#[instrument(name = "git", skip(dir, dry))]
pub fn create(dir: &Path, dry: bool, github_username: Option<String>) -> Result<()> {
    crate::utils::create_dir_gracefully!(dir, dry);

    if !dry {
        // Execute the `cargo init --bin` command in the given directory.
        tracing::debug!("Executing `git init -b main` in {:?}", dir);
        let output = std::process::Command::new("git")
            .arg("init")
            .arg("-b")
            .arg("main")
            .current_dir(dir)
            .output()?;
        tracing::debug!("`git init -b main` output: {:?}", output);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create(&package_dir, false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join(".git").exists());
    }
}
