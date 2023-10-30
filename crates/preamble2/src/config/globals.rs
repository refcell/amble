use clap::ArgAction;

/// Global Pipeline Config Options
#[derive(Debug, Args)]
struct GlobalOpts {
    /// Verbosity level (0-4). Default: 0 (ERROR).
    #[arg(long, short, action = ArgAction::Count, default_value = "0")]
    pub verbosity: u8,

    /// Full generates a full project with license, ci, gitignore, etc included.
    #[arg(long)]
    pub full: bool,

    /// Dry run mode.
    /// If this flag is provided, the cli will not execute commands,
    /// printing the directories and files that would be created instead.
    #[arg(long)]
    pub dry_run: bool,

    /// Overwrite existing files.
    /// If this flag is provided, existing files will be overwritten.
    #[arg(long)]
    pub overwrite: bool,
}
