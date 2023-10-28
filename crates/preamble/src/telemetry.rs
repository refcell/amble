use anyhow::Result;
use tracing::Level;

/// Initializes the tracing subscriber.
///
/// The verbosity level determines the maximum level of tracing.
/// - 0: ERROR
/// - 1: WARN
/// - 2: INFO
/// - 3: DEBUG
/// - 4+: TRACE
///
/// # Arguments
/// * `verbosity_level` - The verbosity level (0-4)
///
/// # Returns
/// * `Result<()>` - Ok if successful, Err otherwise.
pub fn init_tracing_subscriber(verbosity_level: u8) -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(match verbosity_level {
            0 => Level::ERROR,
            1 => Level::WARN,
            2 => Level::INFO,
            3 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow::anyhow!(e))
}
