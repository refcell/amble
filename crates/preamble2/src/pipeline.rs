//! Pipeline
//!
//! todo

/// Pipeline
///
/// The pipeline orchestrates project construction and generation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pipeline {
    /// The verbosity level of the pipeline.
    pub verbosity: u8,
}
