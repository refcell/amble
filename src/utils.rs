use std::path::Path;

use eyre::Result;
use tracing::instrument;

/// Checks if rust artifacts are present in the given directory.
/// If `dry_run` is enabled, this method will not error if rust
/// artifacts are found.
#[instrument(name = "utils", skip(dir, dry_run))]
pub(crate) fn check_artifacts(dir: &Path, dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    if dir.join("Cargo.toml").exists() {
        eyre::bail!("Rust artifacts detected in the project directory");
    }
    Ok(())
}
