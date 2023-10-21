use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// Creates new top-level workspace artifacts at the given directory &[Path].
#[instrument(name = "workspace", skip(dir, name, description, dry, author, tree))]
pub(crate) fn create(
    dir: &Path,
    name: impl AsRef<str> + std::fmt::Display,
    description: Option<impl AsRef<str> + std::fmt::Display>,
    dry: bool,
    author: Option<Vec<String>>,
    overrides: Option<Vec<String>>,
    tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating top level workspace artifacts for {}", name);

    if !dry {
        tracing::debug!("Writing {:?}", dir.join("Cargo.toml"));
        let description = description
            .map(|s| s.as_ref().to_string())
            .unwrap_or(format!("{} workspace", name));
        fill_cargo(
            &dir.join("Cargo.toml"),
            author,
            name.as_ref(),
            &description,
            overrides,
        )?;
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

/// Fetch a packages version using bash commands and `cargo search`.
pub(crate) fn fetch_version(c: &str) -> Option<String> {
    let cargo_search_output = std::process::Command::new("cargo")
        .arg("search")
        .arg(c)
        .output()
        .ok()?;
    if !cargo_search_output.status.success() {
        tracing::warn!("Failed to run `cargo search {}` command", c);
        return None;
    }
    let output_str = String::from_utf8(cargo_search_output.stdout).ok()?;
    let eyre_line = output_str
        .lines()
        .find(|l| l.starts_with(&format!("{} = ", c)))?;
    let version = eyre_line
        .strip_prefix(&format!("{} = \"", c))
        .and_then(|s| s.split('"').next());
    version.map(|s| s.to_string())
}

/// Writes binary contents to the `Cargo.toml` file located at [file].
pub(crate) fn fill_cargo(
    file: &Path,
    author: Option<Vec<String>>,
    name: &str,
    description: &str,
    overrides: Option<Vec<String>>,
) -> Result<()> {
    let mut manifest = toml_edit::Document::new();

    manifest["workspace"] = toml_edit::Item::Table(toml_edit::Table::new());
    let mut array = toml_edit::Array::default();
    array.push("bin/*".to_string());
    array.push("crates/*".to_string());
    manifest["workspace"]["members"] = toml_edit::value(array);
    manifest["workspace"]["resolver"] = toml_edit::value("2");

    manifest["workspace.package"] = toml_edit::Item::Table(toml_edit::Table::new());
    manifest["workspace.package"]["name"] = toml_edit::value(name);
    manifest["workspace.package"]["description"] = toml_edit::value(description);
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

    add_workspace_deps(&mut manifest, overrides);

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

/// Lists the default dependencies.
pub(crate) fn list_dependencies() -> Result<()> {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("Dependency"),
        prettytable::Cell::new("Version"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("eyre"),
        prettytable::Cell::new("0.6.8"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("inquire"),
        prettytable::Cell::new("0.6.2"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("tracing"),
        prettytable::Cell::new("0.1.39"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("serde"),
        prettytable::Cell::new("1.0.189"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("serde_json"),
        prettytable::Cell::new("1.0.107"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("tracing-subscriber"),
        prettytable::Cell::new("0.3.17"),
    ]));
    table.add_row(prettytable::Row::new(vec![
        prettytable::Cell::new("clap"),
        prettytable::Cell::new("4.4.3"),
    ]));
    table.printstd();
    Ok(())
}

/// Add dependencies to the manifest.
pub(crate) fn add_workspace_deps(
    manifest: &mut toml_edit::Document,
    overrides: Option<Vec<String>>,
) {
    let default_inline_dependencies = vec![
        ("eyre".to_string(), "0.6.8".to_string()),
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
    manifest["workspace.dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    add_inline_deps(manifest, combined);
    manifest["workspace.dependencies"]["clap"] =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(toml_edit::InlineTable::new()));
    let version = fetch_version("clap").unwrap_or_else(|| "4.4.3".to_string());
    manifest["workspace.dependencies"]["clap"]["version"] = toml_edit::value(version);
    let mut array = toml_edit::Array::default();
    array.push("derive".to_string());
    manifest["workspace.dependencies"]["clap"]["features"] = toml_edit::value(array);
}

/// Adds inline dependencies to the manifest.
pub(crate) fn add_inline_deps(manifest: &mut toml_edit::Document, deps: Vec<(String, String)>) {
    let deps_table = manifest["workspace.dependencies"].as_table_mut().unwrap();
    for (dep, default_version) in deps {
        let version = fetch_version(&dep).unwrap_or_else(|| default_version.to_string());
        deps_table[&dep] = toml_edit::value(version);
    }
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
    fn test_fetch_version() {
        let version = fetch_version("eyre").unwrap();
        let expected = semver::Version::parse("0.6.8").unwrap();
        let semversion = semver::Version::parse(&version).unwrap();
        // expect as greater than or equal to the expected version
        // since the version may be updated in the future.
        assert!(semversion.gt(&expected) || semversion.eq(&expected));
    }

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
            "example workspace",
            None,
        )
        .unwrap();
        assert!(cargo_toml_path_buf.exists());

        // Validate the cargo.toml file contents
        let mut cargo_toml = File::open(cargo_toml_path_buf).unwrap();
        let mut cargo_toml_contents = String::new();
        cargo_toml.read_to_string(&mut cargo_toml_contents).unwrap();
        let eyre_version = fetch_version("eyre").unwrap_or_else(|| "0.6.8".to_string());
        let inquire_version = fetch_version("inquire").unwrap_or_else(|| "0.6.2".to_string());
        let tracing_version = fetch_version("tracing").unwrap_or_else(|| "0.1.39".to_string());
        let serde_version = fetch_version("serde").unwrap_or_else(|| "1.0.189".to_string());
        let serde_json_version =
            fetch_version("serde_json").unwrap_or_else(|| "1.0.107".to_string());
        let tracing_subscriber_version =
            fetch_version("tracing-subscriber").unwrap_or_else(|| "0.3.17".to_string());
        let clap_version = fetch_version("clap").unwrap_or_else(|| "4.4.3".to_string());
        let expected_contents = format!(
            r#"[workspace]
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
eyre = "{}"
inquire = "{}"
tracing = "{}"
serde = "{}"
serde_json = "{}"
tracing-subscriber = "{}"
clap = {{ version = "{}", features = ["derive"] }}

[profile.dev]
opt-level = 1
overflow-checks = false

[profile.bench]
debug = true
"#,
            eyre_version,
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
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(
            &dir_path_buf,
            "example",
            Some("example workspace"),
            false,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(dir_path_buf.exists());
        assert!(dir_path_buf.join("Cargo.toml").exists());
    }

    #[test]
    fn test_create_dry_run() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(
            &dir_path_buf,
            "example",
            Some("example workspace"),
            true,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(!dir_path_buf.join("Cargo.toml").exists());
    }
}
