use clap::{ArgAction, Parser};
use eyre::Result;
use ptree::TreeBuilder;

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
    #[arg(long)]
    dry_run: bool,

    /// The project name.
    /// This will be used for the binary application name.
    #[arg(long, short, default_value = "example")]
    name: String,

    /// The path to the project directory.
    /// By default, the current working directory is used.
    /// If any rust artifacts are detected in the specified
    /// or unspecified directory, an error will be thrown.
    #[arg(default_value = ".")]
    project_dir: String,
}

/// CLI Entrypoint.
pub fn run() -> Result<()> {
    let Args {
        v,
        dry_run,
        name,
        project_dir,
    } = Args::parse();

    crate::telemetry::init_tracing_subscriber(v)?;

    let mut builder = TreeBuilder::new(project_dir.clone());
    let project_dir_path = std::path::Path::new(&project_dir);
    if !dry_run {
        std::fs::create_dir_all(project_dir_path)?;
    }

    crate::utils::check_artifacts(project_dir_path, dry_run)?;

    crate::root::create(project_dir_path, &name, dry_run, Some(&mut builder))?;
    crate::bins::create(
        &project_dir_path.join("bin"),
        &name,
        dry_run,
        Some(&mut builder),
    )?;
    crate::libs::create(
        &project_dir_path.join("crates"),
        "common",
        dry_run,
        Some(&mut builder),
    )?;

    if dry_run {
        let tree = builder.build();
        ptree::print_tree(&tree).expect("Error printing tree");
    }

    Ok(())
}
