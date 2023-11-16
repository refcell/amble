//! Pipeline
//!
//! todo

use anyhow::Result;

/// Pipeline
///
/// The pipeline orchestrates project construction and generation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pipeline {
    /// The status of the pipeline.
    pub status: PipelineStatus,
}

/// Pipeline Status
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PipelineStatus {
    /// Instantiated, but not yet executed.
    #[default]
    Built,
    /// Executed, but not yet committed.
    Executed,
    /// Committed.
    Committed,
}

impl Pipeline {
    /// Constructs a new PipelineBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes the pipeline.
    pub fn execute(&mut self) -> Result<()> {
        if self.status != PipelineStatus::Built {
            anyhow::bail!("Pipeline has already been executed");
        }
        self.status = PipelineStatus::Executed;
        Ok(())
    }

    /// Commits pipeline changes.
    pub fn commit(&mut self) -> Result<()> {
        match self.status {
            PipelineStatus::Built => anyhow::bail!("Pipeline has not been executed"),
            PipelineStatus::Executed => (),
            PipelineStatus::Committed => anyhow::bail!("Pipeline already committed"),
        }
        self.status = PipelineStatus::Committed;
        Ok(())
    }
}

/// Passthrough
///
/// Declarative macro that derives [`PipelineBuilder`] methods on Pipeline as
/// associated functions. These methods "pass through" to the builder by
/// instantiating a default [`PipelineBuilder`] and calling the corresponding
/// builder method.
///
/// Since the [`PipelineBuilder`] mutator methods consume and return the
/// builder itself, the methods can be chained. Effectively, this allows the
/// [`Pipeline`] to be called like the builder itself.
///
/// # Example
///
/// ```rust
/// use preamble2::Pipeline;
/// let mut pipeline = Pipeline::with_name("example").dry_run(true).build();
/// pipeline.execute().unwrap();
/// pipeline.commit().unwrap();
/// ```
#[macro_export]
macro_rules! passthrough {
    (
        $builder_method:ident,
        $($field_name:ident : $field_type:ty),*
    ) => {
        /// Associated function that creates a [`PipelineBuilder`]
        #[doc = concat!(" and calls the builder's ", stringify!($builder_method), " method.")]
        ///
        /// # Arguments
        ///
        $(#[doc = concat!(" * `", stringify!($field_name), "`: ", stringify!($field_type))])*
        pub fn $builder_method(
            $(
                $field_name : $field_type
            )*
        ) -> $crate::PipelineBuilder {
            $crate::PipelineBuilder::default().$builder_method(
                $($field_name),*
            )
        }
    };

    ($builder_method:ident) => {
        pub fn $builder_method() -> PipelineBuilder {
            $crate::PipelineBuilder::default().$builder_method()
        }
    };
}

impl Pipeline {
    passthrough!(with_name, name: impl Into<String>);
    passthrough!(dry_run, dry: bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        let mut pipeline = Pipeline::new();
        pipeline.execute().unwrap();
        pipeline.commit().unwrap();
    }

    #[test]
    fn test_pipeline_unordered_fails() {
        let mut pipeline = Pipeline::new();
        assert_eq!(pipeline.status, PipelineStatus::Built);

        pipeline.commit().unwrap_err();
        pipeline.execute().unwrap();
        assert_eq!(pipeline.status, PipelineStatus::Executed);

        pipeline.execute().unwrap_err();
        pipeline.commit().unwrap();
        assert_eq!(pipeline.status, PipelineStatus::Committed);

        pipeline.execute().unwrap_err();
        pipeline.commit().unwrap_err();
    }
}
