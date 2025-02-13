use std::path::PathBuf;

use anyhow::{anyhow, Result};
use chumsky::{
    error::Simple,
    prelude::{choice, end, just, take_until},
    Parser,
};
use log::LevelFilter;

use super::config::{LogDestination, LogDestinationConfig};

// TODO Allow lowercase for log levels

const LEVEL_ERROR: &str = "ERROR";
const LEVEL_WARN: &str = "WARN";
const LEVEL_INFO: &str = "INFO";
const LEVEL_DEBUG: &str = "DEBUG";
const LEVEL_TRACE: &str = "TRACE";

const DEST_STDERR: &str = "stderr";
const DEST_SYSLOG: &str = "syslog";
const DEST_FILE: &str = "file";

/// Parse a log definition consisting of an optional log level, and a log destination.
///
/// Format: [level:]destination
/// level = "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE"
/// destination = "stderr" | "syslog" | "file:path"
///
/// Examples:
/// * "syslog"
/// * "stderr"
/// * "file:/path/to/file"
/// * "INFO:stderr"
/// * "DEBUG:file:/path/to/file"
/// * "TRACE:syslog"
pub fn parse_config_definition(input: &str) -> Result<Option<LogDestinationConfig>> {
    config_definition()
        .or_not()
        .then_ignore(end())
        .parse(input)
        .map_err(|err| anyhow!("Failed to parse log config: {err:?}"))
}

fn config_definition() -> impl Parser<char, LogDestinationConfig, Error = Simple<char>> {
    log_level()
        .then_ignore(just(':'))
        .or_not()
        .then(log_destination())
        .map(move |(level, destination)| LogDestinationConfig { level, destination })
}

fn log_level() -> impl Parser<char, LevelFilter, Error = Simple<char>> {
    choice((
        just(LEVEL_ERROR).to(LevelFilter::Error),
        just(LEVEL_WARN).to(LevelFilter::Warn),
        just(LEVEL_INFO).to(LevelFilter::Info),
        just(LEVEL_DEBUG).to(LevelFilter::Debug),
        just(LEVEL_TRACE).to(LevelFilter::Trace),
    ))
}

fn log_destination() -> impl Parser<char, LogDestination, Error = Simple<char>> {
    choice((
        just(DEST_STDERR).to(LogDestination::Stderr),
        just(DEST_SYSLOG).to(LogDestination::Syslog),
        just(DEST_FILE)
            .ignore_then(just(':'))
            .ignore_then(path().map(LogDestination::File)),
    ))
}

fn path() -> impl Parser<char, PathBuf, Error = Simple<char>> {
    take_until(end())
        .then_ignore(end())
        .map(|(chars, ())| String::from_iter(chars).into())
}

fn just_case_insensitive(expected_input: &str) -> impl Parser<char, char, Error = Simple<char>> {
    
    
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest_reuse::{self, *};

    use super::*;

    #[template]
    fn level(
        #[values(
            (LevelFilter::Error, LEVEL_ERROR),
            (LevelFilter::Warn, LEVEL_WARN),
            (LevelFilter::Info, LEVEL_INFO),
            (LevelFilter::Debug, LEVEL_DEBUG),
            (LevelFilter::Trace, LEVEL_TRACE)
        )]
        level: (LevelFilter, &str),
    ) {
    }
    #[template]
    fn destination(
        #[values((LogDestination::Stderr, DEST_STDERR), (LogDestination::Syslog, DEST_SYSLOG))]
        destination: (LogDestination, &str),
    ) {
    }

    #[rstest]
    fn test_empty_config() {
        let config = parse_config_definition("").unwrap();
        assert_eq!(None, config);
    }

    #[apply(level)]
    #[rstest]
    fn test_config_with_only_level(level: (LevelFilter, &str)) {
        let error = parse_config_definition(level.1).unwrap_err();
        assert!(error.to_string().contains("Failed to parse log config"));
    }

    #[apply(destination)]
    #[rstest]
    fn test_with_default_level(destination: (LogDestination, &str)) {
        let config = parse_config_definition(destination.1).unwrap();
        assert_eq!(
            Some(LogDestinationConfig {
                level: None,
                destination: destination.0,
            }),
            config
        );
    }

    #[apply(level)]
    #[apply(destination)]
    #[rstest]
    fn test_with_level(level: (LevelFilter, &str), destination: (LogDestination, &str)) {
        let config = parse_config_definition(&format!("{}:{}", level.1, destination.1)).unwrap();
        assert_eq!(
            Some(LogDestinationConfig {
                level: Some(level.0),
                destination: destination.0
            }),
            config
        );
    }
}
