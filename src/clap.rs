use clap::Parser;

use crate::{LogDestinationConfig, LoggingConfig};

// We need to remove doc comments here, otherwise clap adds them to the help message
#[allow(missing_docs)]
#[derive(Parser, Debug)]
pub struct LogArgs {
    /// Log definition consisting of an optional log level filter, and a log destination.
    /// You can define this argument multiple times for multiple log destinations.
    ///
    /// Logging can be disabled with `--log none`.
    /// If combined with other log definitions, those will take precedence and logging will not be disabled.
    ///
    /// The argument can be combined with a level filter to only log messages of a certain level or higher to that destination.
    ///
    /// Format: destination | level_filter:destination
    /// * level_filter = "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE"
    /// * destination = "stderr" | "syslog" | "file:path" | "none"
    ///
    /// Examples:
    /// * `--log syslog`
    /// * `--log stderr`
    /// * `--log file:/path/to/file`
    /// * `--log INFO:stderr`
    /// * `--log DEBUG:file:/path/to/file`
    /// * `--log TRACE:syslog`
    /// * `--log none`
    #[arg(long, value_parser=parse_destination_config)]
    #[clap(verbatim_doc_comment)]
    pub log: Vec<Option<LogDestinationConfig>>,
}

fn parse_destination_config(input: &str) -> Result<Option<LogDestinationConfig>, String> {
    crate::parser::parse_config_definition(input).map_err(|err| err.to_string())
}

impl LogArgs {
    /// Build the [LoggingConfig] defined by the command line arguments from [LogArgs].
    /// If no `--log` argument is given, the default config is returned.
    pub fn or_default(&self, default: LoggingConfig) -> LoggingConfig {
        if self.log.is_empty() {
            // No `--log` argument given, use the default config
            default
        } else {
            // There are `--log` arguments given, but they may be `--log none`.
            // Let's filter those out. If no non-none are remaining, logging will be disabled.
            let destinations = self.log.iter().filter_map(|log| log.clone()).collect();
            LoggingConfig::new(destinations)
        }
    }
}
