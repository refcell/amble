use std::path::Path;

use anyhow::Result;
use ptree::TreeBuilder;
use tracing::instrument;

/// CI Github Action Workflow String Literal.
pub(crate) const CI_YML: &str = include_str!("../.github/workflows/ci.yml");

/// Audit Github Action Workflow String Literal.
pub(crate) const AUDIT_YML: &str = include_str!("../.github/workflows/audit.yml");

/// Github Release Github Action Workflow String Literal.
pub(crate) const GITHUB_RELEASE_YML: &str = include_str!("../.github/workflows/github-release.yml");

/// Manual Tag Github Action Workflow String Literal.
pub(crate) const MANUAL_TAG_YML: &str = include_str!("../.github/workflows/manual-tag.yml");

/// Release Github Action Workflow String Literal.
pub(crate) const RELEASE_YML: &str = include_str!("../.github/workflows/release.yml");

/// Tag Github Action Workflow String Literal.
pub(crate) const TAG_YML: &str = include_str!("../.github/workflows/tag.yml");

/// Validate Version Github Action Workflow String Literal.
pub(crate) const VALIDATE_VERSION_YML: &str =
    include_str!("../.github/workflows/validate-version.yml");

/// An array of Github Action Workflow String Literals.
pub(crate) const WORKFLOWS: [(&str, &str); 7] = [
    ("ci.yml", CI_YML),
    ("audit.yml", AUDIT_YML),
    ("github-release.yml", GITHUB_RELEASE_YML),
    ("manual-tag.yml", MANUAL_TAG_YML),
    ("release.yml", RELEASE_YML),
    ("tag.yml", TAG_YML),
    ("validate-version.yml", VALIDATE_VERSION_YML),
];

/// Copy all [WORKFLOWS] to the project's `.github/workflows/` directory.
#[instrument(name = "workflows", skip(dir, dry, tree))]
pub(crate) fn write_github_workflows(
    dir: &Path,
    dry: bool,
    tree: &mut Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Writing github workflows");
    for (workflow_name, workflow_contents) in WORKFLOWS.iter() {
        let workflow_path_buf = dir.join(workflow_name);
        if !dry {
            tracing::debug!("Writing {:?}", workflow_path_buf);
            std::fs::write(&workflow_path_buf, workflow_contents)?;
        }
        tree.as_deref_mut()
            .map(|t| t.add_empty_child(workflow_name.to_string()));
    }
    Ok(())
}

/// Creates ci workflows for github actions.
#[instrument(name = "ci", skip(dir, dry, ci, tree))]
pub(crate) fn create(
    dir: &Path,
    dry: bool,
    ci: Option<String>,
    mut tree: Option<&mut TreeBuilder>,
) -> Result<()> {
    tracing::info!("Creating ci");

    let workflows_dir = dir.join(".github").join("workflows");
    let ci_yml_path_buf = workflows_dir.join("ci.yml");
    crate::utils::create_dir_gracefully!(&workflows_dir, dry);

    tree.as_deref_mut()
        .map(|t| t.begin_child(".github".to_string()));
    tree.as_deref_mut()
        .map(|t| t.begin_child("workflows".to_string()));

    if ci.is_none() {
        tracing::debug!("Writing {:?}", ci_yml_path_buf);
        write_github_workflows(&workflows_dir, dry, &mut tree)?;
    }
    if !dry && ci.is_some() {
        tracing::debug!(
            "Copying {:?} to {:?}",
            ci.as_ref().unwrap(),
            ci_yml_path_buf
        );
        std::fs::copy(ci.as_ref().unwrap(), ci_yml_path_buf)?;
        tree.as_deref_mut()
            .map(|t| t.add_empty_child(ci.unwrap()));
    }

    tree.as_deref_mut().map(|t| t.end_child()); // <- workflows/
    tree.map(|t| t.end_child()); // <- .github/

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_github_workflows() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        write_github_workflows(&dir_path_buf, false, &mut None).unwrap();
        for (workflow_name, workflow_contents) in WORKFLOWS.iter() {
            let workflow_path_buf = dir_path_buf.join(workflow_name);
            assert!(workflow_path_buf.exists());
            let workflow_contents_string = std::fs::read_to_string(&workflow_path_buf).unwrap();
            assert_eq!(workflow_contents_string, *workflow_contents);
        }
    }

    #[test]
    fn test_create() {
        let dir = tempdir().unwrap();
        let dir_path_buf = dir.path().to_path_buf();
        create(&dir_path_buf, false, None, None).unwrap();
        let workflows_dir = dir_path_buf.join(".github").join("workflows");
        assert!(workflows_dir.exists());
        let ci_yml_path_buf = workflows_dir.join("ci.yml");
        assert!(ci_yml_path_buf.exists());
        let ci_yml_contents = std::fs::read_to_string(&ci_yml_path_buf).unwrap();
        assert_eq!(ci_yml_contents, CI_YML);
    }
}
