use std::io::Write;
use std::path::Path;

use eyre::Result;
use ptree::TreeBuilder;
use tracing::instrument;

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

    crate::utils::create_dir_gracefully!(workflows_dir, dry);

    tree.as_deref_mut()
        .map(|t| t.begin_child(".github".to_string()));
    tree.as_deref_mut()
        .map(|t| t.begin_child("workflows".to_string()));

    if !dry && ci.is_none() {
        tracing::debug!("Writing {:?}", ci_yml_path_buf);
        write_ci_yml(&ci_yml_path_buf)?;
    }
    if !dry && ci.is_some() {
        tracing::debug!(
            "Copying {:?} to {:?}",
            ci.as_ref().unwrap(),
            ci_yml_path_buf
        );
        std::fs::copy(ci.unwrap(), ci_yml_path_buf)?;
    }
    tree.as_deref_mut()
        .map(|t| t.add_empty_child("ci.yml".to_string()));

    tree.as_deref_mut().map(|t| t.end_child()); // <- workflows/
    tree.map(|t| t.end_child()); // <- .github/

    Ok(())
}

/// Writes ci contents to the `ci.yml` file located at [file].
pub(crate) fn write_ci_yml(file: &Path) -> Result<()> {
    let mut ci_yml = std::fs::File::create(file)?;
    let ci_contents = r#"name: CI

on:
  push:
  pull_request:
  workflow_dispatch:

permissions:
  contents: read

env:
  RUSTFLAGS: -Dwarnings

jobs:
  cargo-tests:
    runs-on: ubuntu-latest
    timeout-minutes: 20
    steps:
      - name: Checkout sources
        uses: actions/checkout@v3
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          profile: minimal
          override: true
      - uses: Swatinem/rust-cache@v1
        with:
          cache-on-failure: true
      - name: cargo test
        run: cargo test --all
      - name: cargo test all features
        run: cargo test --all --all-features
  cargo-lint:
    runs-on: ubuntu-latest
    timeout-minutes: 20
    steps:
      - name: Checkout sources
        uses: actions/checkout@v3
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          profile: minimal
          components: rustfmt, clippy
          override: true
      - uses: Swatinem/rust-cache@v1
        with:
          cache-on-failure: true
      - name: cargo fmt
        run: cargo +nightly fmt --all -- --check
      - name: cargo clippy
        run: cargo +nightly clippy --all --all-features -- -D warnings
  cargo-build:
    runs-on: ubuntu-latest
    timeout-minutes: 20
    continue-on-error: true
    steps:
      - name: Checkout sources
        uses: actions/checkout@v3
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          profile: minimal
          override: true
      - uses: Swatinem/rust-cache@v1
        with:
          cache-on-failure: true
      - name: build
        id: build
        continue-on-error: true
        run: cargo build --all
  cargo-doc:
    runs-on: ubuntu-latest
    timeout-minutes: 20
    continue-on-error: true
    steps:
      - name: Checkout sources
        uses: actions/checkout@v3
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          profile: minimal
          override: true
      - uses: Swatinem/rust-cache@v1
        with:
          cache-on-failure: true
      - name: doclint
        id: build
        continue-on-error: true
        run: RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --all-features --document-private-items
      - name: doctest
        run: cargo test --doc --all --all-features"#;
    ci_yml.write_all(ci_contents.as_bytes())?;
    Ok(())
}
