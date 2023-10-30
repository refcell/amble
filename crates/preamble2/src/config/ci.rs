use clap::ArgAction;

/// CI Pipeline Config Options
#[derive(Debug, Args)]
struct CIOpts {
    /// Adds all github actions ci workflows.
    #[arg(long, short)]
    pub with_ci: bool,

    /// Adds an audit ci workflow.
    #[arg(long)]
    pub with_audit_ci: bool,

    /// Adds rust testing ci workflow.
    #[arg(long)]
    pub with_rust_ci: bool,

    /// Adds a github release ci workflow.
    #[arg(long)]
    pub with_github_release_ci: bool,

    /// Adds a manual tag ci workflow.
    #[arg(long)]
    pub with_manual_tag_ci: bool,

    /// Adds a release ci workflow.
    #[arg(long)]
    pub with_release_ci: bool,

    /// Adds a tag ci workflow.
    #[arg(long)]
    pub with_tag_ci: bool,

    /// Adds a ci workflow that validates the project's cargo version against the latest tagged
    /// version.
    #[arg(long)]
    pub with_cargo_version_ci: bool,

    /// Copy the specified workflow file to the project's `.github/workflows/` directory.
    #[arg(long, short)]
    pub ci_yml: Option<String>,
}
