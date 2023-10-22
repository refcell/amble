use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates a new cargo binary project in the given directory.
#[instrument(name = "bin", skip(dir, dry, tree))]
pub(crate) fn create_bin(dir: &Path, dry: bool, mut tree: Option<&mut TreeBuilder>) -> Result<()> {
    crate::utils::create_dir_gracefully!(dir, dry);
    if !dry {
        // Execute the `cargo init --bin` command in the given directory.
        tracing::debug!("Executing `cargo init --bin` in {:?}", dir);
        let output = std::process::Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .current_dir(dir)
            .output()?;
        tracing::debug!("cargo init --bin output: {:?}", output);
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));
    tree.as_deref_mut()
        .map(|t| t.begin_child("src".to_string()));
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("main.rs".to_string()));
    tree.map(|t| t.end_child());
    Ok(())
}

/// Creates a new cargo library project in the given directory.
#[instrument(name = "lib", skip(dir, dry, tree))]
pub(crate) fn create_lib(dir: &Path, dry: bool, mut tree: Option<&mut TreeBuilder>) -> Result<()> {
    crate::utils::create_dir_gracefully!(dir, dry);
    if !dry {
        // Execute the `cargo init --lib` command in the given directory.
        tracing::debug!("Executing `cargo init --lib` in {:?}", dir);
        let output = std::process::Command::new("cargo")
            .arg("init")
            .arg("--lib")
            .current_dir(dir)
            .output()?;
        tracing::debug!("cargo init --lib output: {:?}", output);
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));
    tree.as_deref_mut()
        .map(|t| t.begin_child("src".to_string()));
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("lib.rs".to_string()));
    tree.map(|t| t.end_child());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_bin() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create_bin(&package_dir, false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("src").exists());
        assert!(package_dir.join("src").join("main.rs").exists());
        assert!(package_dir.join("Cargo.toml").exists());
    }

    #[test]
    fn test_create_lib() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create_lib(&package_dir, false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("src").exists());
        assert!(package_dir.join("src").join("lib.rs").exists());
        assert!(package_dir.join("Cargo.toml").exists());
    }
}
