/*!
Pipeline Configuration

The pipeline configuration is a set of flattened [clap] argument structs
that are used to configure the pipeline.

*/

use anyhow::Result;
use clap::{ArgAction, Parser};

pub use clap::Parser;

mod ci;
mod root;
mod misc;
mod globals;

/// Pipeline Config Object
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[clap(flatten)]
    pub global_opts: globals::GlobalOpts,

    #[clap(flatten)]
    pub ci_opts: ci::CIOpts,

    #[clap(flatten)]
    pub misc_opts: misc::MiscOpts,

    #[clap(flatten)]
    pub root_opts: root::RootOpts,

    #[clap(subcommand)]
    pub ptype: PipelineType,
}

/// Pipeline Type
#[derive(Subcommand, Debug)]
pub enum PipelineType {
    /// List the default dependencies.
    List,

    /// Generates a workspace project.
    /// This is the default project type.
    Workspace,

    /// Builds a cargo binary project.
    Bin {
        /// Bare mode. If specified, only the basic `cargo init` files will
        /// be generated.
        #[arg(long)]
        bare: bool,
    },

    /// Builds a cargo library project.
    Lib {
        /// Bare mode. If specified, only the basic `cargo init` files will
        /// be generated.
        #[arg(long)]
        bare: bool,
    },
}
