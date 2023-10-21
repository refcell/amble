#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/amble/main/etc/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/refcell/amble/main/etc/favicon.ico",
    issue_tracker_base_url = "https://github.com/refcell/amble/issues/"
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// The CLI Module
pub mod cli;

pub(crate) mod bins;
pub(crate) mod cargo;
pub(crate) mod ci;
pub(crate) mod gitignore;
pub(crate) mod libs;
pub(crate) mod license;
pub(crate) mod root;
pub(crate) mod telemetry;
pub(crate) mod utils;
