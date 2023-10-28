// =============================================================================

/*!
A library for building and managing rust workspaces with batteries included.
Preamble is used by [amble](https://crates.io/crates/amble) for its core
functionality.

# Overview

Contains a number of modules for building rust workspaces with batteries included.

# Usage

```ignore,rust
use preamble2::telemetry;

fn main() {
    // Initialize the tracing subscriber with default 0 verbosity (only errors)
    telemetry::init_default_tracing().unwrap();
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
    pipeline::Pipeline,
    telemetry::{
        init_default_tracing, init_tracing_subscriber, init_tracing_subscriber_with_env, Level,
    },
};

pub mod pipeline;
pub mod telemetry;

// =============================================================================
