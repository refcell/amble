use anyhow::Result;
use clap::{ArgAction, Parser};
use inquire::Confirm;
use ptree::TreeBuilder;

use preamble::{bins, cargo, ci, etc, gitignore, libs, license, root, telemetry, utils};

/// Command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Verbosity level (0-4).
    #[arg(long, short, action = ArgAction::Count, default_value = "0")]
    v: u8,

    /// Dry run mode.
    /// If this flag is provided, the cli will not execute commands,
    /// printing the directories and files that would be created instead.
    #[arg(long)]
    dry_run: bool,

    /// Overwrite existing files.
    /// If this flag is provided, the cli will overwrite existing files.
    #[arg(long)]
    overwrite: bool,

    /// Bare mode. Only for `--bin` and `--lib` flags. If specified,
    /// generated files will be the basic `cargo init` files.
    #[arg(long)]
    bare: bool,

    /// The project name.
    /// This will be used for the binary application name.
    #[arg(long, short, default_value = "example")]
    name: String,

    /// Add github actions ci workflow.
    #[arg(long, short)]
    with_ci: bool,

    /// Copy the specified ci workflow file to the project's `.github/workflows/` directory.
    #[arg(long, short)]
    ci_yml: Option<String>,

    /// Override the project authors.
    #[arg(long, short)]
    authors: Option<Vec<String>>,

    /// Builds a cargo binary project.
    #[arg(long, short)]
    bin: bool,

    /// Builds a cargo library project.
    #[arg(long, short)]
    lib: bool,

    /// Prevents a readme from being generated.
    #[arg(long)]
    without_readme: bool,

    /// Full generates a full project structure including license, ci, gitignore, etc.
    #[arg(long)]
    full: bool,

    /// Adds an `etc/` directory to the project.
    /// This _Et Cetera_ directory is used for storing miscellaneous files.
    #[arg(long)]
    etc: bool,

    /// Adds template assets to the `etc/` directory of the generate project.
    /// Will be run automatically if the `--full` flag is provided.
    #[arg(long)]
    assets: bool,

    /// Adds an MIT License to the project.
    /// The MIT License type can be overridden with the `--with-license` flag.
    #[arg(long)]
    license: bool,

    /// Adds a Gitignore file to the project.
    #[arg(long)]
    gitignore: bool,

    /// Specifies the description of the project in the top-level `Cargo.toml` workspace.
    #[arg(long, short)]
    description: Option<String>,

    /// Adds these dependencies to the top-level `Cargo.toml` workspace
    /// alongside the default dependencies.
    #[arg(long)]
    dependencies: Option<Vec<String>>,

    /// Lists the default dependencies.
    #[arg(long)]
    list: bool,

    /// License Override.
    /// This will override the default MIT License.
    /// The license type must be a valid SPDX license identifier.
    #[arg(long)]
    with_license: Option<String>,

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
        mut assets,
        bare,
        without_readme,
        name,
        project_dir,
        mut overwrite,
        mut with_ci,
        ci_yml,
        authors,
        bin,
        lib,
        mut license,
        with_license,
        mut gitignore,
        full,
        description,
        list,
        dependencies,
        mut etc,
    } = Args::parse();
    let project_dir_path = std::path::Path::new(&project_dir);

    if full {
        with_ci = true;
        license = true;
        gitignore = true;
        etc = true;
        assets = true;
    }

    if list {
        root::list_dependencies()?;
        return Ok(());
    }

    telemetry::init_tracing_subscriber(v)?;

    match overwrite {
        true => {
            tracing::warn!("Overwrite flag is set, existing files will be overwritten");
            if !Confirm::new("[WARNING] Overwrite mode will overwrite any conflicting files and directories. Are you sure you wish to proceed?").prompt()? {
                println!("Phew, close call... aborting");
                return Ok(());
            }
        }
        false => {
            utils::check_artifacts(project_dir_path, with_ci || ci_yml.is_some(), dry_run)?;
            overwrite = true;
        }
    }

    // we don't need to prompt the user twice if overwrite mode is enabled
    if !dry_run && !overwrite {
        tracing::warn!("Running in non-dry run mode.");
        tracing::warn!("This action may be destructive.");
        if !Confirm::new("Running amble in non-dry mode, are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            return Ok(());
        }
    }

    let mut builder = TreeBuilder::new(project_dir.clone());
    if !dry_run {
        std::fs::create_dir_all(project_dir_path)?;
    }

    if license || with_license.is_some() {
        let license_type = with_license.as_deref().unwrap_or("mit");
        license::create(project_dir_path, license_type, dry_run, Some(&mut builder))?;
    }

    if gitignore {
        gitignore::create(project_dir_path, dry_run, Some(&mut builder))?;
    }

    if etc {
        etc::create(project_dir_path, dry_run, assets, Some(&mut builder))?;
    }

    if !bin && !lib {
        root::create(
            project_dir_path,
            &name,
            description.as_ref(),
            dry_run,
            without_readme,
            authors,
            dependencies,
            Some(&mut builder),
        )?;
        bins::create(
            &project_dir_path.join("bin"),
            &name,
            dry_run,
            Some(&mut builder),
        )?;
        libs::create(
            &project_dir_path.join("crates"),
            "common",
            dry_run,
            Some(&mut builder),
        )?;
    } else if bin {
        cargo::create_bin(
            project_dir_path,
            &name,
            description.as_ref(),
            dry_run,
            bare,
            authors,
            dependencies,
            Some(&mut builder),
        )?;
    } else if lib {
        cargo::create_lib(
            project_dir_path,
            &name,
            description.as_ref(),
            dry_run,
            bare,
            authors,
            dependencies,
            Some(&mut builder),
        )?;
    }

    if with_ci || ci_yml.is_some() {
        ci::create(project_dir_path, dry_run, ci_yml, Some(&mut builder))?;
    }

    if dry_run {
        let tree = builder.build();
        ptree::print_tree(&tree).expect("Error printing tree");
    }

    Ok(())
}
