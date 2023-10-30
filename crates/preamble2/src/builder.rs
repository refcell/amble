/*!
PipelineBuilder

The PipelineBuilder is a builder for the Pipeline. It is used to configure the
Pipeline before it is executed. The PipelineBuilder is consumed when the
Pipeline is built, and the Pipeline is consumed when it is executed. This

# Usage

```rust
use preamble2::PipelineBuilder;

let mut builder = PipelineBuilder::new();
builder = builder.dry_run(true);
builder = builder.with_name("example");

let mut pipeline = builder.build();
pipeline.execute().unwrap();
pipeline.commit().unwrap();
```
*/

/// Pipeline Builder
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PipelineBuilder {
    /// The verbosity level of the pipeline.
    pub verbosity: u8,

    /// Dry run
    pub dry_run: bool,

    /// With input output 
    pub name: Option<String>,

    /// The internal Pipeline [crate::Config].
    pub config: crate::Config,
}

impl PipelineBuilder {
    /// Constructs a new PipelineBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses the pipeline from the command line.
    pub fn parse() -> Self {
        Self { config: Config::parse(), ..Self::default() }
    }

    /// Sets the pipeline to use dry run mode.
    pub fn dry_run(self, dry: bool) -> Self {
        Self { dry_run: dry, ..self }
    }

    /// Sets the project name.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self { name: Some(name.into()), ..self }
    }

    /// Builds the pipeline.
    pub fn build(self) -> crate::Pipeline {
        crate::Pipeline {
            status: crate::PipelineStatus::Built,
            // verbosity: self.verbosity,
            // dry_run: self.dry_run,
            // name: self.name,
        }
    }

    /// Creates the pipeline and executes it, bubbling up
    /// any errors and returning the pipeline.
    pub fn execute(self) -> Result<crate::Pipeline> {
        let mut pipeline = self.build();
        pipeline.execute()?;
        Ok(pipeline)
    }
}
