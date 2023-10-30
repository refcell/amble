// =============================================================================

/*!
A library for building and managing rust workspaces with batteries included.
Preamble is used by [amble](https://crates.io/crates/amble) for its core
functionality.

# Overview

Contains a number of modules for building rust workspaces with batteries included.

# Usage

The core `preamble2`

```ignore,rust
use preamble2::Pipeline;

fn main() {
    // Initializes a tracing subscriber with 0 verbosity
    // (only prints errors)
    preamble2::init_default_tracing().unwrap();

    let mut pipeline = Pipeline::default();
    pipeline.with_root();
    pipeline.dry();


    pipeline.execute().unwrap();

    // Commit will be a no-op here since we
    // set the pipeline to be in dry run mode
    pipeline.commit().unwrap();
    tracing::info!("Pipeline finished successfully");
}
```
*/

// =============================================================================

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/amble/main/etc/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/refcell/amble/main/etc/favicon.ico",
    issue_tracker_base_url = "https://github.com/refcell/amble/issues/"
)]
#![warn(missing_debug_implementations, unreachable_pub)]
#![warn(rustdoc::all)]
#![deny(unused_must_use, missing_docs, rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

// =============================================================================

pub use crate::{
    builder::PipelineBuilder,
    pipeline::{Pipeline, PipelineStatus},
    telemetry::{
        init_default_tracing, init_tracing_subscriber, init_tracing_subscriber_with_env, Level,
    },
    #[cfg(feature = "clap")]
    config::{Config, Parser},
};

pub mod builder;
pub mod pipeline;
pub mod telemetry;

#[cfg(feature = "clap")]
pub mod config;

// =============================================================================
