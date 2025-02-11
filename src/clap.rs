use clap::Parser;

use crate::LogDestinationConfig;

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
    pub log: Vec<LogDestinationConfig>,
}

fn parse_destination_config(input: &str) -> Result<LogDestinationConfig, String> {
    crate::parser::parse_config_definition(input)
        .map_err(|err| err.to_string())
        .and_then(|config| config.ok_or_else(|| "Failed to parse log config".to_string()))
}

impl From<LogArgs> for crate::config::LoggingConfig {
    fn from(args: LogArgs) -> Self {
        if args.log.is_empty() {
            Self::LoggingDisabled
        } else {
            Self::LoggingEnabled {
                destinations: args.log,
            }
        }
    }
}
