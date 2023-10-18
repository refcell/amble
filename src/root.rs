use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates new top-level workspace artifacts at the given directory &[Path].
#[instrument(name = "workspace", skip(dir, name, dry, tree))]
pub(crate) fn create(
    dir: &Path,
    name: impl AsRef<str> + std::fmt::Display,
    dry: bool,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating top level workspace artifacts for {}", name);

    if !dry {
        tracing::debug!(
            "Creating crate Cargo.toml file as {:?}",
            dir.join("Cargo.toml")
        );
        let mut cargo_toml = std::fs::File::create(dir.join("Cargo.toml"))?;
        cargo_toml.write_all(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/templates/Cargo.toml"
        )))?;
    }
    tree.map(|t| t.add_empty_child("Cargo.toml".to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(&dir_path_buf, "example", false, None).unwrap();
        assert!(dir_path_buf.exists());
        assert!(dir_path_buf.join("Cargo.toml").exists());
    }

    #[test]
    fn test_create_dry_run() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(&dir_path_buf, "example", true, None).unwrap();
        assert!(!dir_path_buf.join("Cargo.toml").exists());
    }
}
