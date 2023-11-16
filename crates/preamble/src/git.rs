use anyhow::Result;
use ptree::TreeBuilder;
use std::{path::Path, process::Command};
use tracing::instrument;

/// Constructs the git repository url from the given github username and repository name.
pub fn build_repository_url(github_username: &str, repository_name: &str) -> String {
    format!("https://github.com/{}/{}", github_username, repository_name)
}

/// Constructs the git repository url with a `.git` suffix from the given github username and
/// repository name.
pub fn build_git_remote_target(github_username: &str, repository_name: &str) -> String {
    format!("{}.git", build_repository_url(github_username, repository_name))
}
/// Attempts to retrieve the current git username.
pub fn try_git_username() -> Option<String> {
    match Command::new("git").arg("config").arg("--get").arg("user.name").output() {
        Ok(output) => {
            if output.status.success() {
                let name = String::from_utf8(output.stdout).ok()?;
                Some(name.trim().to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Create git repository with given github username. If github username is not specified, try to
/// grab one
#[instrument(name = "git", skip(dir, dry, user, tree))]
pub fn create(
    dir: &Path,
    dry: bool,
    user: Option<String>,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
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
    tree.map(|t| t.add_empty_child(".git".to_string()));

    if !dry {
        // Setting the remote origin for the git repository.
        let origin = build_git_remote_target(
            &user.unwrap_or_else(|| try_git_username().unwrap_or_default()),
            dir.file_name().unwrap().to_str().unwrap(),
        );
        tracing::debug!("Executing `git remote add origin {}` in {:?}", origin, dir);
        let output = std::process::Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&origin)
            .current_dir(dir)
            .output()?;
        tracing::debug!("`git remote add origin` output: {:?}", output);
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
        create(&package_dir, false, None, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join(".git").exists());
    }
}
