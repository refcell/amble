use eyre::Result;
use ptree::TreeBuilder;
use std::io::Write;
use std::path::Path;
use tracing::instrument;

/// Creates a new gitignore file in the given directory.
#[instrument(name = "gitignore", skip(dir, dry, tree))]
pub(crate) fn create(dir: &Path, dry: bool, tree: Option<&mut TreeBuilder>) -> Result<()> {
    tracing::info!("Creating a .gitignore file");

    // Create the directory if it doesn't exist.
    if !dry {
        tracing::debug!("Creating directory {:?}", dir);
        std::fs::create_dir_all(dir)?;
    }

    if !dry {
        tracing::debug!("Writing .gitignore to {:?}", dir.join(".gitignore"));
        let mut file = std::fs::File::create(dir.join(".gitignore"))?;
        let rust_gitignore = gitignores::Root::Rust.to_string();
        file.write_all(rust_gitignore.as_bytes())?;
    }

    tree.map(|t| t.add_empty_child(".gitignore".to_string()));
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
        assert!(package_dir.join(".gitignore").exists());
    }
}
