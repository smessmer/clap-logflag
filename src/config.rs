use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggingConfig {
    // TODO It might be better to remove LoggingDisabled and instead represent it as an empty destination vector.
    LoggingDisabled,
    LoggingEnabled {
        destinations: Vec<LogDestinationConfig>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogDestination {
    /// Log to stderr
    Stderr,

    /// Log to the file at the given path
    File(PathBuf),

    /// Log to the system log
    Syslog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogDestinationConfig {
    pub destination: LogDestination,

    /// Only log messages at this level or higher to this destination.
    ///
    /// If `None`, the default level is used.
    pub level: Option<log::LevelFilter>,
}
