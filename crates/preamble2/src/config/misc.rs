use clap::ArgAction;

/// Misc Pipeline Config Options
#[derive(Debug, Args)]
struct MiscOpts {
    /// Adds an `etc/` directory to the project.
    /// This directory is used for storing miscellaneous files.
    #[arg(long)]
    pub etc: bool,

    /// Adds template assets to the `etc/` directory of the generate project.
    /// Will be run automatically if the `--full` flag is provided.
    #[arg(long)]
    pub assets: bool,
}
