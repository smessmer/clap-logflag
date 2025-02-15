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

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_destination_config {
        use crate::LogDestination;

        use super::*;

        #[test]
        fn empty_string() {
            assert_eq!(
                parse_destination_config(""),
                Err(
                    "Invalid empty log destination. Choose stderr, syslog, file, or none"
                        .to_string()
                )
            );
        }

        #[test]
        fn none() {
            assert_eq!(parse_destination_config("none"), Ok(None));
        }

        #[test]
        fn stderr() {
            assert_eq!(
                parse_destination_config("stderr"),
                Ok(Some(LogDestinationConfig {
                    destination: LogDestination::Stderr,
                    level: None
                }))
            );
        }

        #[test]
        fn stderr_with_level() {
            assert_eq!(
                parse_destination_config("DEBUG:stderr"),
                Ok(Some(LogDestinationConfig {
                    destination: LogDestination::Stderr,
                    level: Some(log::LevelFilter::Debug)
                }))
            );
        }
    }

    mod or_default {
        use crate::LogDestination;

        use super::*;

        #[test]
        fn no_flags_present_chooses_default() {
            let args = LogArgs { log: vec![] };
            let default = vec![LogDestinationConfig {
                destination: LogDestination::Stderr,
                level: Some(log::LevelFilter::Info),
            }];
            let parsed = args.or_default(LoggingConfig::new(default.clone()));
            assert_eq!(default, parsed.destinations());
        }

        #[test]
        fn none_flag_present() {
            let args = LogArgs { log: vec![None] };
            let parsed = args.or_default(LoggingConfig::new(vec![LogDestinationConfig {
                destination: LogDestination::Stderr,
                level: Some(log::LevelFilter::Info),
            }]));
            assert_eq!(parsed.destinations().len(), 0);
        }

        #[test]
        fn one_flag_present() {
            let destinations = vec![LogDestinationConfig {
                destination: LogDestination::Stderr,
                level: Some(log::LevelFilter::Info),
            }];
            let args = LogArgs {
                log: destinations.iter().cloned().map(Some).collect(),
            };
            let parsed = args.or_default(LoggingConfig::new(vec![]));
            assert_eq!(destinations, parsed.destinations());
        }

        #[test]
        fn two_flags_present() {
            let destinations = vec![
                LogDestinationConfig {
                    destination: LogDestination::Stderr,
                    level: Some(log::LevelFilter::Info),
                },
                LogDestinationConfig {
                    destination: LogDestination::File(std::path::PathBuf::from("/tmp/logfile")),
                    level: Some(log::LevelFilter::Debug),
                },
            ];
            let args = LogArgs {
                log: destinations.iter().cloned().map(Some).collect(),
            };
            let parsed = args.or_default(LoggingConfig::new(vec![]));
            assert_eq!(destinations, parsed.destinations());
        }

        #[test]
        fn two_flags_with_one_none_present() {
            let first_flag = LogDestinationConfig {
                destination: LogDestination::Stderr,
                level: Some(log::LevelFilter::Info),
            };
            let destinations = vec![Some(first_flag.clone()), None];
            let args = LogArgs { log: destinations };
            let parsed = args.or_default(LoggingConfig::new(vec![]));
            assert_eq!(vec![first_flag], parsed.destinations());
        }
    }
}
