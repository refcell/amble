use std::io::Write;
use std::path::Path;

use anyhow::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates a new cargo binary project in the given directory.
#[instrument(name = "bin", skip(dir, name, description, dry, author, tree))]
pub(crate) fn create_bin(
    dir: &Path,
    name: impl AsRef<str> + std::fmt::Display,
    description: Option<impl AsRef<str> + std::fmt::Display>,
    dry: bool,
    author: Option<Vec<String>>,
    overrides: Option<Vec<String>>,
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
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

        tracing::debug!("Filling cargo contents in {:?}", dir);
        write_cargo_bin(
            &dir.join("Cargo.toml"),
            author,
            name.as_ref(),
            &description
                .map(|d| d.to_string())
                .unwrap_or_else(|| "A new binary crate".to_string()),
            overrides,
        )?;
        tracing::debug!("Finished filling cargo contents in {:?}", dir);
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

/// Writes to the binary `Cargo.toml` file located at [file].
pub(crate) fn write_cargo_bin(
    file: &Path,
    author: Option<Vec<String>>,
    name: &str,
    description: &str,
    overrides: Option<Vec<String>>,
) -> Result<()> {
    let mut manifest = toml_edit::Document::new();
    manifest["package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["package"]["name"] = toml_edit::value(name);
    manifest["package"]["description"] = toml_edit::value(description);
    manifest["package"]["version"] = toml_edit::value("0.1.0");
    manifest["package"]["edition"] = toml_edit::value("2021");
    manifest["package"]["license"] = toml_edit::value("MIT");
    let user = crate::root::get_current_username(&author);
    manifest["package"]["authors"] = crate::root::get_authors(author);
    manifest["package"]["repository"] =
        toml_edit::value(format!("https://github.com/{}/{}", user, name));
    manifest["package"]["homepage"] =
        toml_edit::value(format!("https://github.com/{}/{}", user, name));

    add_inline_deps(&mut manifest, overrides);

    let mut file = std::fs::File::create(file)?;
    file.write_all(manifest.to_string().as_bytes())?;

    Ok(())
}

/// Add dependencies to the manifest.
pub(crate) fn add_inline_deps(manifest: &mut toml_edit::Document, overrides: Option<Vec<String>>) {
    let default_inline_dependencies = vec![
        ("anyhow".to_string(), "1.0".to_string()),
        ("inquire".to_string(), "0.6.2".to_string()),
        ("tracing".to_string(), "0.1.39".to_string()),
        ("serde".to_string(), "1.0.189".to_string()),
        ("serde_json".to_string(), "1.0.107".to_string()),
        ("tracing-subscriber".to_string(), "0.3.17".to_string()),
        ("clap".to_string(), "4.4.3".to_string()),
    ];
    let combined = match overrides {
        Some(v) => {
            let mut combined = default_inline_dependencies;
            let override_deps = v.into_iter().map(|s| (s, "0.0.0".to_string()));
            combined.extend(override_deps);
            combined
        }
        None => default_inline_dependencies,
    };
    manifest["dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    let deps_table = manifest["dependencies"].as_table_mut().unwrap();
    for (dep, default_version) in combined {
        let version =
            crate::root::fetch_version(&dep).unwrap_or_else(|| default_version.to_string());
        deps_table[&dep] = toml_edit::value(version);
    }
    manifest["dependencies"]["clap"] =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(toml_edit::InlineTable::new()));
    let version = crate::root::fetch_version("clap").unwrap_or_else(|| "4.4.3".to_string());
    manifest["dependencies"]["clap"]["version"] = toml_edit::value(version);
    let mut array = toml_edit::Array::default();
    array.push("derive".to_string());
    manifest["dependencies"]["clap"]["features"] = toml_edit::value(array);
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
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_write_cargo_bin() {
        use crate::root::fetch_version;

        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let proj_name = "example";
        let cargo_toml_path_buf = dir_path_buf.join("Cargo.toml");
        write_cargo_bin(
            &cargo_toml_path_buf,
            Some(vec!["refcell".to_string()]),
            proj_name,
            "example binary",
            None,
        )
        .unwrap();
        assert!(cargo_toml_path_buf.exists());

        // Validate the cargo.toml file contents
        let mut cargo_toml = File::open(cargo_toml_path_buf).unwrap();
        let mut cargo_toml_contents = String::new();
        cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();
        let anyhow_version = fetch_version("anyhow").unwrap_or_else(|| "1.0".to_string());
        let inquire_version = fetch_version("inquire").unwrap_or_else(|| "0.6.2".to_string());
        let tracing_version = fetch_version("tracing").unwrap_or_else(|| "0.1.39".to_string());
        let serde_version = fetch_version("serde").unwrap_or_else(|| "1.0.189".to_string());
        let serde_json_version =
            fetch_version("serde_json").unwrap_or_else(|| "1.0.107".to_string());
        let tracing_subscriber_version =
            fetch_version("tracing-subscriber").unwrap_or_else(|| "0.3.17".to_string());
        let clap_version = fetch_version("clap").unwrap_or_else(|| "4.4.3".to_string());
        let expected_contents = format!(
            r#"[package]
name = "example"
description = "example binary"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["refcell"]
repository = "https://github.com/refcell/example"
homepage = "https://github.com/refcell/example"

[dependencies]
anyhow = "{}"
inquire = "{}"
tracing = "{}"
serde = "{}"
serde_json = "{}"
tracing-subscriber = "{}"
clap = {{ version = "{}", features = ["derive"] }}
"#,
            anyhow_version,
            inquire_version,
            tracing_version,
            serde_version,
            serde_json_version,
            tracing_subscriber_version,
            clap_version
        );
        assert_eq!(cargo_toml_contents, expected_contents);
    }

    #[test]
    fn test_create_bin() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let package_dir = dir_path_buf.join("example");
        create_bin(
            &package_dir,
            "example",
            Some("example binary"),
            false,
            None,
            None,
            None,
        )
        .unwrap();

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
