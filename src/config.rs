use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggingConfig {
    LoggingDisabled,
    LoggingEnabled {
        destinations: Vec<LogDestinationConfig>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogDestination {
    Stderr,
    File(PathBuf),
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
