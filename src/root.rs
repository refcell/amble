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
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating top level workspace artifacts for {}", name);

    let cargo_toml_path_buf = dir.join("Cargo.toml");
    let crates_dir_path_buf = dir.join("crates");
    let bin_dir_path_buf = dir.join("bin");

    if !dry {
        tracing::debug!(
            "Creating crate Cargo.toml file as {:?}",
            cargo_toml_path_buf
        );
        let mut cargo_toml = std::fs::File::create(&cargo_toml_path_buf)?;
        cargo_toml.write_all(include_bytes!("../templates/Cargo.toml"))?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));

    if !dry {
        tracing::debug!("Creating crates directory at {:?}", crates_dir_path_buf);
        std::fs::create_dir_all(&crates_dir_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("crates".to_string()));

    if !dry {
        tracing::debug!("Creating crates directory at {:?}", bin_dir_path_buf);
        std::fs::create_dir_all(&bin_dir_path_buf)?;
    }
    tree.map(|t| t.add_empty_child("bin".to_string()));

    // tree.as_deref_mut().map(|t| t.end_child()); // <- src/
    // tree.as_deref_mut().map(|t| t.end_child()); // <- <name>/

    Ok(())
}
