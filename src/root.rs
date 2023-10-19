use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates new top-level workspace artifacts at the given directory &[Path].
#[instrument(name = "workspace", skip(dir, name, dry, author, tree))]
pub(crate) fn create(
    dir: &Path,
    name: impl AsRef<str> + std::fmt::Display,
    dry: bool,
    author: Option<Vec<String>>,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating top level workspace artifacts for {}", name);

    if !dry {
        tracing::debug!("Writing {:?}", dir.join("Cargo.toml"));
        fill_cargo(&dir.join("Cargo.toml"), author, name.as_ref())?;
    }
    tree.map(|t| t.add_empty_child("Cargo.toml".to_string()));

    Ok(())
}

/// Attempts to retrieve the current git username.
pub(crate) fn try_git_username() -> Option<String> {
    match std::process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let name = String::from_utf8(output.stdout).ok()?;
                Some(name.trim().to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Returns the current username.
pub(crate) fn get_current_username(authors: &Option<Vec<String>>) -> String {
    match authors {
        Some(v) => v[0].clone(),
        None => match try_git_username() {
            Some(name) => name,
            None => whoami::username().to_string(),
        },
    }
}

/// Dynamically gets the authors.
pub(crate) fn get_authors(authors: Option<Vec<String>>) -> toml_edit::Item {
    let mut array = toml_edit::Array::default();
    match authors {
        Some(v) => v.into_iter().for_each(|a| array.push(a)),
        None => match try_git_username() {
            Some(name) => array.push(name),
            None => array.push(whoami::username().to_string()),
        },
    };
    toml_edit::value(array)
}

/// Writes binary contents to the `Cargo.toml` file located at [file].
pub(crate) fn fill_cargo(file: &Path, author: Option<Vec<String>>, name: &str) -> Result<()> {
    let mut manifest = toml_edit::Document::new();

    manifest["workspace"] = toml_edit::Item::Table(toml_edit::Table::new());
    let mut array = toml_edit::Array::default();
    array.push("bin/*".to_string());
    array.push("crates/*".to_string());
    manifest["workspace"]["members"] = toml_edit::value(array);
    manifest["workspace"]["resolver"] = toml_edit::value("2");

    manifest["workspace.package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["workspace.package"]["name"] = toml_edit::value(name);
    manifest["workspace.package"]["description"] = toml_edit::value(format!("{} workspace", name));
    manifest["workspace.package"]["version"] = toml_edit::value("0.1.0");
    manifest["workspace.package"]["edition"] = toml_edit::value("2021");
    manifest["workspace.package"]["license"] = toml_edit::value("MIT");
    let user = get_current_username(&author);
    manifest["workspace.package"]["authors"] = get_authors(author);
    manifest["workspace.package"]["repository"] =
        toml_edit::value(format!("https://github.com/{}/{}", user, name));
    manifest["workspace.package"]["homepage"] =
        toml_edit::value(format!("https://github.com/{}/{}", user, name));
    let mut array = toml_edit::Array::default();
    array.push("**/target".to_string());
    array.push("benches/".to_string());
    array.push("tests".to_string());
    manifest["workspace.package"]["exclude"] = toml_edit::value(array);

    manifest["workspace.dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["workspace.dependencies"]["eyre"] = toml_edit::value("0.6.8");
    manifest["workspace.dependencies"]["inquire"] = toml_edit::value("0.6.2");
    manifest["workspace.dependencies"]["tracing"] = toml_edit::value("0.1.39");
    manifest["workspace.dependencies"]["serde"] = toml_edit::value("1.0.189");
    manifest["workspace.dependencies"]["serde_json"] = toml_edit::value("1.0.107");
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

    // Remove quotes inside toml table keys.
    // And write the manifest to the Cargo TOML file.
    let mut file = std::fs::File::create(file)?;
    let manifest_string = remove_table_quotes(manifest.to_string());
    file.write_all(manifest_string.as_bytes())?;

    Ok(())
}

/// Removes quotes from table keys.
/// e.g. ["workspace.package"] -> [workspace.package"]
pub(crate) fn remove_table_quotes(s: String) -> String {
    let re = regex::Regex::new(r#"\["(.*\..*)"\]"#).unwrap_or_else(|_| panic!("Invalid regex"));
    let result = re.replace_all(&s, |caps: &regex::Captures<'_>| format!("[{}]", &caps[1]));
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_remove_table_quotes() {
        let s = r#"[workspace.package]"#;
        let expected = r#"[workspace.package]"#;
        assert_eq!(remove_table_quotes(s.to_string()), expected);

        let s = r#"["workspace.package"]"#;
        let expected = r#"[workspace.package]"#;
        assert_eq!(remove_table_quotes(s.to_string()), expected);

        let s = r#"["refcell"]"#;
        let expected = r#"["refcell"]"#;
        assert_eq!(remove_table_quotes(s.to_string()), expected);

        let s = r#"["**/target", "benches/", "tests"]"#;
        let expected = r#"["**/target", "benches/", "tests"]"#;
        assert_eq!(remove_table_quotes(s.to_string()), expected);

        let s = r#"["workspace.package.inside"]"#;
        let expected = r#"[workspace.package.inside]"#;
        assert_eq!(remove_table_quotes(s.to_string()), expected);
    }

    #[test]
    fn test_fill_cargo() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        let proj_name = "example";
        let cargo_toml_path_buf = dir_path_buf.join("Cargo.toml");
        fill_cargo(
            &cargo_toml_path_buf,
            Some(vec!["refcell".to_string()]),
            proj_name,
        )
        .unwrap();
        assert!(cargo_toml_path_buf.exists());

        // Validate the cargo.toml file contents
        let mut cargo_toml = File::open(cargo_toml_path_buf).unwrap();
        let mut cargo_toml_contents = String::new();
        cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();
        let expected_contents = r#"[workspace]
members = ["bin/*", "crates/*"]
resolver = "2"

[workspace.package]
name = "example"
description = "example workspace"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["refcell"]
repository = "https://github.com/refcell/example"
homepage = "https://github.com/refcell/example"
exclude = ["**/target", "benches/", "tests"]

[workspace.dependencies]
eyre = "0.6.8"
inquire = "0.6.2"
tracing = "0.1.39"
serde = "1.0.189"
serde_json = "1.0.107"
tracing-subscriber = "0.3.17"
clap = { version = "4.4.3", features = ["derive"] }

[profile.dev]
opt-level = 1
overflow-checks = false

[profile.bench]
debug = true
"#;
        assert_eq!(cargo_toml_contents, expected_contents);
    }

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(&dir_path_buf, "example", false, None, None).unwrap();
        assert!(dir_path_buf.exists());
        assert!(dir_path_buf.join("Cargo.toml").exists());
    }

    #[test]
    fn test_create_dry_run() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(&dir_path_buf, "example", true, None, None).unwrap();
        assert!(!dir_path_buf.join("Cargo.toml").exists());
    }
}
