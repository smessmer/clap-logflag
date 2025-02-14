use std::path::PathBuf;

/// This enum represents the whole logging configuration,
/// including all logging destinations and their respective log level filters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggingConfig {
    // TODO It might be better to remove LoggingDisabled and instead represent it as an empty destination vector.
    LoggingDisabled,
    LoggingEnabled {
        destinations: Vec<LogDestinationConfig>,
    },
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
