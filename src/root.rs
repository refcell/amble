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
        tracing::debug!("Writing {:?}", dir.join("Cargo.toml"));
        fill_cargo(&dir.join("Cargo.toml"), name.as_ref())?;
    }
    tree.map(|t| t.add_empty_child("Cargo.toml".to_string()));

    Ok(())
}

/// Writes binary contents to the `Cargo.toml` file located at [file].
pub(crate) fn fill_cargo(file: &Path, name: &str) -> Result<()> {
    let mut manifest = toml_edit::Document::new();

    manifest["workspace"] = toml_edit::Item::Table(toml_edit::Table::new());
    let mut array = toml_edit::Array::default();
    array.push(format!("bin/{}", name));
    array.push("crates/*".to_string());
    manifest["workspace"]["members"] = toml_edit::value(array);
    manifest["workspace"]["resolver"] = toml_edit::value("2");

    manifest["workspace.package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["workspace.package"]["name"] = toml_edit::value(name);
    manifest["workspace.package"]["description"] = toml_edit::value(format!("{} workspace", name));
    manifest["workspace.package"]["version"] = toml_edit::value("0.1.0");
    manifest["workspace.package"]["edition"] = toml_edit::value("2021");
    // todo: get the author from git?
    manifest["workspace.package"]["authors"] = toml_edit::value("The Rust Project Developers");
    manifest["workspace.package"]["license"] = toml_edit::value("MIT OR Apache-2.0");
    // todo: fetch the reposiroty and homepage from git?
    manifest["workspace.package"]["repository"] = toml_edit::value("");
    manifest["workspace.package"]["homepage"] = toml_edit::value("");
    let mut array = toml_edit::Array::default();
    array.push("**/target".to_string());
    array.push("benches/".to_string());
    array.push("tests".to_string());
    manifest["workspace.package"]["exclude"] = toml_edit::value(array);

    // todo: fetch these dynamically like in
    //       https://github.com/rust-lang/cargo/blob/master/src/cargo/ops/cargo_add/mod.rs
    // todo: allow a cli flag to specify a list of dependencies to add
    manifest["workspace.dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["workspace.dependencies"]["eyre"] = toml_edit::value("0.6.8");
    manifest["workspace.dependencies"]["inquire"] = toml_edit::value("0.6.2");
    manifest["workspace.dependencies"]["tracing"] = toml_edit::value("0.1.39");
    manifest["workspace.dependencies"]["tracing-subscriber"] = toml_edit::value("0.3.17");
    manifest["workspace.dependencies"]["clap"] =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(toml_edit::InlineTable::new()));
    manifest["workspace.dependencies"]["clap"]["version"] = toml_edit::value("4.4.3");
    let mut array = toml_edit::Array::default();
    array.push("derive".to_string());
    manifest["workspace.dependencies"]["clap"]["features"] = toml_edit::value(array);

    manifest["profile.dev"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["profile.dev"]["opt-level"] = toml_edit::value(1);
    manifest["profile.dev"]["overflow-checks"] = toml_edit::value(false);

    manifest["profile.bench"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["profile.bench"]["debug"] = toml_edit::value(true);

    // Write the manifest to the file.
    let mut file = std::fs::File::create(file)?;
    file.write_all(manifest.to_string().as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
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
        let expected_contents = r#"[workspace]
members = ["bin/example", "crates/*"]
resolver = "2"

["workspace.package"]
name = "example"
description = "example workspace"
version = "0.1.0"
edition = "2021"
authors = "The Rust Project Developers"
license = "MIT OR Apache-2.0"
repository = ""
homepage = ""
exclude = ["**/target", "benches/", "tests"]

["workspace.dependencies"]
eyre = "0.6.8"
inquire = "0.6.2"
tracing = "0.1.39"
tracing-subscriber = "0.3.17"
clap = { version = "4.4.3", features = ["derive"] }

["profile.dev"]
opt-level = 1
overflow-checks = false

["profile.bench"]
debug = true
"#;
        assert_eq!(cargo_toml_contents, expected_contents);
    }

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
