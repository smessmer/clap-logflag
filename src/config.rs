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
    pub level: log::LevelFilter,
}
