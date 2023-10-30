use anyhow::Result;
use inquire::Confirm;
use ptree::TreeBuilder;

use preamble2::{Pipeline, Config, Parser};

/// CLI Entrypoint.
pub fn run() -> Result<()> {
    let mut pipeline = Pipeline::parse();
    pipeline.execute()?;
    pipeline.commit()?;
    Ok(())

    .with_name("example").dry_run(true).build();
    pipeline.execute().unwrap();
    pipeline.commit().unwrap();

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
            if !Confirm::new("[WARNING] Overwrite mode will overwrite any conflicting files. Are you sure you wish to proceed?").prompt()? {
                println!("Phew, close call... aborting");
                return Ok(());
            }
        }
        false => {
            utils::check_artifacts(project_dir_path, with_ci || ci_yml.is_some(), dry_run)?;
            overwrite = true;
        }
    }

    // Don't need to prompt the user twice if overwrite mode is enabled
    if !dry_run && !overwrite {
        tracing::warn!("Running in non-dry run mode.");
        tracing::warn!("This action may be destructive.");
        if !Confirm::new("Running amble in without dry mode, are you sure you wish to proceed?")
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
        bins::create(&project_dir_path.join("bin"), &name, dry_run, Some(&mut builder))?;
        libs::create(&project_dir_path.join("crates"), "common", dry_run, Some(&mut builder))?;
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
