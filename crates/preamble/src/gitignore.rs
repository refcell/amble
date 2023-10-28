use anyhow::Result;
use ptree::TreeBuilder;
use std::{io::Write, path::Path};
use tracing::instrument;

/// Creates a new gitignore file in the given directory.
#[instrument(name = "gitignore", skip(dir, dry, tree))]
pub fn create(dir: &Path, dry: bool, tree: Option<&mut TreeBuilder>) -> Result<()> {
    tracing::info!("Creating a .gitignore file");
    crate::utils::create_dir_gracefully!(dir, dry);

    if !dry {
        tracing::debug!("Writing .gitignore to {:?}", dir.join(".gitignore"));
        let mut file =
            std::fs::File::options().append(true).create(true).open(dir.join(".gitignore"))?;
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

        // Get the content lengths of the file
        // and make sure that if we create again,
        // the content length increases since the file
        // is opened in append mode.
        let first_content_length = package_dir.join(".gitignore").metadata().unwrap().len();
        create(&package_dir, false, None).unwrap();
        let second_content_length = package_dir.join(".gitignore").metadata().unwrap().len();
        assert_eq!(second_content_length, 2 * first_content_length);
    }
}
