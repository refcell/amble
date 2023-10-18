use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates a new bin crate.
#[instrument(name = "bin", skip(dir, name, dry, tree))]
pub(crate) fn create(
    dir: &Path,
    name: impl AsRef<str>,
    dry: bool,
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating binary crate");

    let project_path_buf = dir.join(name.as_ref());
    let cargo_toml_path_buf = project_path_buf.join("Cargo.toml");
    let src_path_buf = project_path_buf.join("src");
    let main_rs_path_buf = project_path_buf.join("src").join("main.rs");

    if !dry {
        tracing::debug!("Creating bin directory at {:?}", dir);
        std::fs::create_dir_all(dir)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child("bin".to_string()));

    if !dry {
        tracing::debug!("Creating crate directory at {:?}", project_path_buf);
        std::fs::create_dir_all(&project_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child(name.as_ref().to_string()));

    if !dry {
        tracing::debug!(
            "Creating crate Cargo.toml file as {:?}",
            cargo_toml_path_buf
        );
        let mut cargo_toml = std::fs::File::create(&cargo_toml_path_buf)?;
        cargo_toml.write_all(include_bytes!("../templates/bin/Cargo.toml"))?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("Cargo.toml".to_string()));

    if !dry {
        tracing::debug!("Creating crate src directory at {:?}", src_path_buf);
        std::fs::create_dir_all(&src_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.begin_child("src".to_string()));

    if !dry {
        tracing::debug!("Creating main.rs file as {:?}", main_rs_path_buf);
        let mut main_rs = std::fs::File::create(&main_rs_path_buf)?;
        main_rs.write_all(include_bytes!("../templates/bin/main.rs"))?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("main.rs".to_string()));

    tree.as_deref_mut().map(|t| t.end_child()); // <- src/
    tree.as_deref_mut().map(|t| t.end_child()); // <- <name>/
    tree.map(|t| t.end_child()); // <- bin/

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let bin_path_buf = dir_path_buf.join("bin");
        let project_name = "example";
        let project_path = bin_path_buf.join(project_name);
        create(&bin_path_buf, project_name, false, None).unwrap();

        assert!(project_path.exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("src").join("main.rs").exists());
        assert!(project_path.join("Cargo.toml").exists());

        let mut main_rs = File::open(project_path.join("src").join("main.rs")).unwrap();
        let mut main_rs_contents = String::new();
        main_rs.read_to_string(&mut main_rs_contents).unwrap();
        let expected_contents = "fn main() {\n    println!(\"Hello World!\");\n}\n";
        assert_eq!(main_rs_contents, expected_contents);
    }

    #[test]
    fn test_create_dry_run() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let bin_path_buf = dir_path_buf.join("bin");
        let project_name = "example";
        let project_path = bin_path_buf.join(project_name);
        create(&bin_path_buf, project_name, true, None).unwrap();

        assert!(!project_path.exists());
        assert!(!project_path.join("src").exists());
        assert!(!project_path.join("src").join("main.rs").exists());
        assert!(!project_path.join("Cargo.toml").exists());
    }
}
