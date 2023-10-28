use std::{io::Write, path::Path};

use anyhow::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates a new bin crate.
#[instrument(name = "bin", skip(dir, name, dry, tree))]
pub fn create(
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

    crate::utils::create_dir_gracefully!(src_path_buf, dry);

    tree.as_deref_mut().map(|t| t.begin_child("bin".to_string()));
    tree.as_deref_mut().map(|t| t.begin_child(name.as_ref().to_string()));

    if !dry {
        tracing::debug!("Writing {:?}", cargo_toml_path_buf);
        fill_cargo(&cargo_toml_path_buf, name.as_ref())?;
    }
    tree.as_deref_mut().map(|t| t.add_empty_child("Cargo.toml".to_string()));
    tree.as_deref_mut().map(|t| t.begin_child("src".to_string()));

    if !dry {
        tracing::debug!("Writing {:?}", main_rs_path_buf);
        let mut main_rs = std::fs::File::create(&main_rs_path_buf)?;
        let main_contents = "fn main() {\n    println!(\"Hello World!\");\n}\n";
        main_rs.write_all(main_contents.as_bytes())?;
    }
    tree.as_deref_mut().map(|t| t.add_empty_child("main.rs".to_string()));

    tree.as_deref_mut().map(|t| t.end_child()); // <- src/
    tree.as_deref_mut().map(|t| t.end_child()); // <- <name>/
    tree.map(|t| t.end_child()); // <- bin/

    Ok(())
}

/// Writes binary contents to the `Cargo.toml` file located at [file].
pub fn fill_cargo(file: &Path, name: &str) -> Result<()> {
    let mut manifest = toml_edit::Document::new();
    manifest["package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["package"]["name"] = toml_edit::value(name);
    manifest["package"]["description"] = toml_edit::value(format!("{} cli binary", name));
    let inline =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(toml_edit::InlineTable::new()));
    manifest["package"]["version"] = inline.clone();
    manifest["package"]["version"]["workspace"] = toml_edit::value(true);
    manifest["package"]["edition"] = inline.clone();
    manifest["package"]["edition"]["workspace"] = toml_edit::value(true);
    manifest["package"]["authors"] = inline.clone();
    manifest["package"]["authors"]["workspace"] = toml_edit::value(true);
    manifest["package"]["license"] = inline.clone();
    manifest["package"]["license"]["workspace"] = toml_edit::value(true);
    manifest["package"]["repository"] = inline.clone();
    manifest["package"]["repository"]["workspace"] = toml_edit::value(true);
    manifest["package"]["homepage"] = inline.clone();
    manifest["package"]["homepage"]["workspace"] = toml_edit::value(true);

    manifest["dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["dependencies"]["common"] = inline.clone();
    manifest["dependencies"]["common"]["path"] = toml_edit::value("../../crates/common");
    manifest["dependencies"]["clap"] = inline.clone();
    manifest["dependencies"]["clap"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["anyhow"] = inline.clone();
    manifest["dependencies"]["anyhow"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["inquire"] = inline.clone();
    manifest["dependencies"]["inquire"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["tracing"] = inline.clone();
    manifest["dependencies"]["tracing"]["workspace"] = toml_edit::value(true);
    manifest["dependencies"]["tracing-subscriber"] = inline.clone();
    manifest["dependencies"]["tracing-subscriber"]["workspace"] = toml_edit::value(true);

    let mut file = std::fs::File::create(file)?;
    file.write_all(manifest.to_string().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};
    use tempfile::tempdir;

    #[test]
    fn test_fill_cargo() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let proj_name = "example";
        let cargo_toml_path_buf = dir_path_buf.join("Cargo.toml");
        fill_cargo(&cargo_toml_path_buf, proj_name).unwrap();
        assert!(cargo_toml_path_buf.exists());

        // Validate the cargo.toml file contents
        let mut cargo_toml = File::open(cargo_toml_path_buf).unwrap();
        let mut cargo_toml_contents = String::new();
        cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();
        let expected_contents = r#"[package]
name = "example"
description = "example cli binary"
version = { workspace = true }
edition = { workspace = true }
authors = { workspace = true }
license = { workspace = true }
repository = { workspace = true }
homepage = { workspace = true }

[dependencies]
common = { path = "../../crates/common" }
clap = { workspace = true }
anyhow = { workspace = true }
inquire = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
"#;
        assert_eq!(cargo_toml_contents, expected_contents);
    }

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
