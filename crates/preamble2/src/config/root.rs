use clap::ArgAction;

/// Root Level Pipeline Config Options
#[derive(Debug, Args)]
struct RootOpts {
    /// The path to the project directory.
    /// By default, the current working directory is used.
    #[arg(default_value = ".")]
    pub directory: String,

    /// The rust project name.
    #[arg(long, short, default_value = "example")]
    pub name: String,

    /// Specifies the project description.
    /// This is used in the top-level README.md and Cargo.toml.
    #[arg(long, short)]
    pub description: Option<String>,

    /// Override the project authors.
    #[arg(long, short)]
    pub authors: Option<Vec<String>>,

    /// Adds a Gitignore file to the project.
    #[arg(long)]
    pub gitignore: bool,

    /// Adds an MIT License to the project.
    /// The MIT License type can be overridden with the `--with-license` flag.
    #[arg(long)]
    pub license: bool,

    /// License Override.
    /// This will override the default MIT License.
    /// The license type must be a valid SPDX license identifier.
    #[arg(long)]
    pub with_license: Option<String>,

    /// Adds these dependencies to the top-level `Cargo.toml` workspace
    /// alongside the default dependencies.
    #[arg(long)]
    pub dependencies: Option<Vec<String>>,

    /// Prevents a readme from being generated or overwritten.
    #[arg(long)]
    pub without_readme: bool,
}
