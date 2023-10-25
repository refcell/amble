use anyhow::Result;
use ptree::TreeBuilder;
use std::path::Path;
use tracing::instrument;

/// Creates a new etc directory in the given directory.
#[instrument(name = "etc", skip(dir, dry, tree))]
pub(crate) fn create(dir: &Path, dry: bool, tree: Option<&mut TreeBuilder>) -> Result<()> {
    tracing::info!("Creating etc directory");
    crate::utils::create_dir_gracefully!(dir.join("etc"), dry);
    tree.map(|t| t.add_empty_child("etc".to_string()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_gitignore() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create(&package_dir, false, None).unwrap();

        assert!(package_dir.exists());
        assert!(package_dir.join("etc").exists());

        // Check that the etc directory is an empty directory
        assert!(package_dir.join("etc").read_dir().unwrap().next().is_none());
    }
}
