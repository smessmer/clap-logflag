use clap::Parser;

use crate::{LogDestinationConfig, LoggingConfig};

#[derive(Parser, Debug)]
pub struct LogArgs {
    // TODO Formatting of this is weird in `--help` output
    // TODO Mention how to disable logging
    /// Log definition consisting of an optional log level, and a log destination.
    /// You can define this argument multiple times for multiple log destinations.
    ///
    /// Format: \[level:\]destination
    ///
    /// level = "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE"
    ///
    /// destination = "stderr" | "syslog" | "file:path"
    ///
    /// Examples:
    /// * "syslog"
    /// * "stderr"
    /// * "file:/path/to/file"
    /// * "INFO:stderr"
    /// * "DEBUG:file:/path/to/file"
    /// * "TRACE:syslog"
    #[arg(long, value_parser=parse_destination_config)]
    pub log: Vec<Option<LogDestinationConfig>>,
}

fn parse_destination_config(input: &str) -> Result<Option<LogDestinationConfig>, String> {
    crate::parser::parse_config_definition(input).map_err(|err| err.to_string())
}

impl LogArgs {
    pub fn or_default(&self, default: LoggingConfig) -> LoggingConfig {
        if self.log.is_empty() {
            // No `--log` argument given, use the default config
            default
        } else {
            // There are `--log` arguments given, but they may be `--log none`.
            // Let's filter those out
            let destinations: Vec<LogDestinationConfig> =
                self.log.iter().filter_map(|log| log.clone()).collect();
            if destinations.is_empty() {
                // All `--log` arguments were `--log none`, disable logging
                LoggingConfig::LoggingDisabled
            } else {
                // There was at least one `--log` argument that wasn't `--log none`, enable logging
                LoggingConfig::LoggingEnabled { destinations }
            }
        }
    }
}
