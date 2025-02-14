use std::path::PathBuf;

/// This enum represents the whole logging configuration,
/// including all logging destinations and their respective log level filters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggingConfig {
    /// List of destinations to log to.
    /// If the list of destinations is empty, logging is disabled.
    destinations: Vec<LogDestinationConfig>,
}

impl LoggingConfig {
    /// Create a new [LoggingConfig] with the given destinations.
    ///
    /// If the list of destinations is empty, logging is disabled.
    pub fn new(destinations: Vec<LogDestinationConfig>) -> Self {
        Self { destinations }
    }

    /// Create a [LoggingConfig] that disables logging.
    pub fn disabled() -> Self {
        Self {
            destinations: vec![],
        }
    }

    /// Get the list of destinations to log to.
    pub fn destinations(&self) -> &[LogDestinationConfig] {
        &self.destinations
    }
}

/// Configuration for a log destination, containing the destination and the log level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogDestinationConfig {
    /// The destination to log to.
    pub destination: LogDestination,

    /// Only log messages at this level or higher to this destination.
    ///
    /// If `None`, the default level is used.
    pub level: Option<log::LevelFilter>,
}

/// A destination that can be logged to, e.g. a file or the system log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogDestination {
    /// Log to stderr
    Stderr,

    /// Log to the file at the given path
    File(PathBuf),

    /// Log to the system log
    Syslog,
}
