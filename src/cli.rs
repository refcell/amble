use clap::{ArgAction, Parser};
use eyre::Result;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Verbosity level (0-4).
    #[arg(long, short, action = ArgAction::Count, default_value = "2")]
    v: u8,

    /// Dry run mode.
    /// If this flag is provided, the cli will no execute commands,
    /// printing the directories and files that would be created instead.
    #[arg(long, short)]
    dry_run: bool,

    /// The project name.
    /// This will be used for the binary application name.
    #[arg(long, short, default_value = "example")]
    project_name: String,
}

/// CLI Entrypoint.
pub fn run() -> Result<()> {
    let Args {
        v,
        dry_run,
        project_name,
    } = Args::parse();

    crate::telemetry::init_tracing_subscriber(v)?;

    tracing::info!("running amble with project name: {}", project_name);
    tracing::debug!("dry run mode: {}", dry_run);

    Ok(())
}
