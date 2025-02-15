use log::LevelFilter;
use std::fmt::{Display, Formatter};

use super::config::{LogDestination, LogDestinationConfig};

const LEVEL_ERROR: &str = "error";
const LEVEL_ERROR_UPPER: &str = "ERROR";
const LEVEL_WARN: &str = "warn";
const LEVEL_WARN_UPPER: &str = "WARN";
const LEVEL_INFO: &str = "info";
const LEVEL_INFO_UPPER: &str = "INFO";
const LEVEL_DEBUG: &str = "debug";
const LEVEL_DEBUG_UPPER: &str = "DEBUG";
const LEVEL_TRACE: &str = "trace";
const LEVEL_TRACE_UPPER: &str = "TRACE";

const DEST_STDERR: &str = "stderr";
const DEST_SYSLOG: &str = "syslog";
const DEST_FILE: &str = "file";
const DEST_NONE: &str = "none";

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

enum Token {
    Level(TokenLevel),
    Destination(TokenDestination),
}

enum TokenLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

enum TokenDestination {
    Stderr,
    Syslog,
    File,
    None,
}

impl Token {
    fn parse(input: &str) -> Option<Self> {
        match input.to_ascii_lowercase().as_str() {
            LEVEL_ERROR => Some(Token::Level(TokenLevel::Error)),
            LEVEL_WARN => Some(Token::Level(TokenLevel::Warn)),
            LEVEL_INFO => Some(Token::Level(TokenLevel::Info)),
            LEVEL_DEBUG => Some(Token::Level(TokenLevel::Debug)),
            LEVEL_TRACE => Some(Token::Level(TokenLevel::Trace)),
            DEST_STDERR => Some(Token::Destination(TokenDestination::Stderr)),
            DEST_SYSLOG => Some(Token::Destination(TokenDestination::Syslog)),
            DEST_FILE => Some(Token::Destination(TokenDestination::File)),
            DEST_NONE => Some(Token::Destination(TokenDestination::None)),
            _ => None,
        }
    }
}

/// Parse a log definition consisting of an optional log level, and a log destination.
///
/// Format: [level:]destination
/// level = "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE"
/// destination = "stderr" | "syslog" | "file:path" | "none"
///
/// Examples:
/// * "syslog"
/// * "stderr"
/// * "none"
/// * "file:/path/to/file"
/// * "INFO:stderr"
/// * "DEBUG:file:/path/to/file"
/// * "TRACE:syslog"
pub fn parse_config_definition(input: &str) -> Result<Option<LogDestinationConfig>, ParseError> {
    let parts: Vec<&str> = input.split(':').collect();
    assert!(
        !parts.is_empty(),
        "Splitting should always return at least one part"
    );

    let first_token = Token::parse(parts[0]);

    match first_token {
        Some(Token::Destination(destination)) => {
            // If the first component is a destination, parse all the remaining ones as extras
            parse_config_definition_without_level(destination, &parts[1..])
        }
        Some(Token::Level(level)) => {
            // Otherwise, if the first component is a level, parse the second one as a destination and the rest as extras.
            if parts.len() >= 2 {
                parse_config_definition_with_level(level, parts[0], parts[1], &parts[2..])
            } else {
                // Seems we only have a level, no destination.
                Err(ParseError::new(format!(
                    "Expected log destination but found level filter `{}`. Please add a destination. Example: `--log {}:stderr`", parts[0], parts[0],
                )))
            }
        }
        None => {
            if parts.len() == 1 {
                // We only have one part, no colons. Let's assume the user wanted to write a log destination.
                if parts[0].is_empty() {
                    Err(ParseError::new(format!(
                        "Invalid empty log destination. Choose {DEST_STDERR}, {DEST_SYSLOG}, {DEST_FILE}, or {DEST_NONE}"
                    )))
                } else {
                    Err(ParseError::new(format!(
                        "Invalid log destination `{input}`. Choose {DEST_STDERR}, {DEST_SYSLOG}, {DEST_FILE}, or {DEST_NONE}"
                    )))
                }
            } else {
                // We have multiple parts. Let's check what the second part is for a better error message
                match Token::parse(parts[1]) {
                    Some(Token::Destination(_)) => {
                        // The second part is a destination, so let's assume the user wanted to write a level filter in the first part.
                        let error = if parts[0].is_empty() {
                            ParseError::new(format!(
                                "Invalid empty log level filter. Choose {LEVEL_ERROR_UPPER}, {LEVEL_WARN_UPPER}, {LEVEL_INFO_UPPER}, {LEVEL_DEBUG_UPPER}, or {LEVEL_TRACE_UPPER}"
                            ))
                        } else {
                            ParseError::new(format!(
                                "Invalid log level filter `{}`. Choose {LEVEL_ERROR_UPPER}, {LEVEL_WARN_UPPER}, {LEVEL_INFO_UPPER}, {LEVEL_DEBUG_UPPER}, or {LEVEL_TRACE_UPPER}",
                                parts[0]
                            ))
                        };
                        Err(error)
                    }
                    _ => {
                        // The second part is not a destination either. Let's just show a generic error message
                        Err(ParseError::new(format!(
                            "Invalid log configuration `{input}`. Examples: `{DEST_STDERR}`, `{LEVEL_ERROR_UPPER}:{DEST_SYSLOG}`, `{LEVEL_WARN_UPPER}:{DEST_FILE}:/path/to/file`",
                        )))
                    }
                }
            }
        }
    }
}

fn parse_config_definition_without_level(
    destination: TokenDestination,
    extras: &[&str],
) -> Result<Option<LogDestinationConfig>, ParseError> {
    let destination = parse_destination(None, destination, extras)?;
    Ok(destination.map(|destination| LogDestinationConfig {
        level: None,
        destination,
    }))
}

fn parse_config_definition_with_level(
    level: TokenLevel,
    level_str: &str,
    destination: &str,
    extras: &[&str],
) -> Result<Option<LogDestinationConfig>, ParseError> {
    let level_filter = parse_level(level)?;
    let destination = tokenize_and_parse_destination(Some(level_str), destination, extras)?;
    Ok(destination.map(|destination| LogDestinationConfig {
        level: Some(level_filter),
        destination,
    }))
}

fn parse_level(level: TokenLevel) -> Result<LevelFilter, ParseError> {
    match level {
        TokenLevel::Error => Ok(LevelFilter::Error),
        TokenLevel::Warn => Ok(LevelFilter::Warn),
        TokenLevel::Info => Ok(LevelFilter::Info),
        TokenLevel::Debug => Ok(LevelFilter::Debug),
        TokenLevel::Trace => Ok(LevelFilter::Trace),
    }
}

fn tokenize_and_parse_destination(
    level: Option<&str>,
    destination: &str,
    extras: &[&str],
) -> Result<Option<LogDestination>, ParseError> {
    match Token::parse(destination) {
        Some(Token::Destination(destination)) => parse_destination(level, destination, extras),
        Some(Token::Level(_)) => {
            Err(ParseError::new(format!(
                "Expected log destination but found level filter `{destination}`. Please add a destination. Example: `--log {destination}:stderr`"
            )))
        }
        None => {
            let error = if destination.is_empty() {
                ParseError::new(format!(
                    "Invalid empty log destination. Choose {DEST_STDERR}, {DEST_SYSLOG}, {DEST_FILE}, or {DEST_NONE}"
                ))
            } else {
                ParseError::new(format!(
                "Invalid log destination `{destination}`. Choose {DEST_STDERR}, {DEST_SYSLOG}, {DEST_FILE}, or {DEST_NONE}"
            ))
            };
            Err(error)
        }
    }
}

fn parse_destination(
    level: Option<&str>,
    destination: TokenDestination,
    extras: &[&str],
) -> Result<Option<LogDestination>, ParseError> {
    let destination = match destination {
        TokenDestination::Stderr => Some(LogDestination::Stderr),
        TokenDestination::Syslog => Some(LogDestination::Syslog),
        TokenDestination::None => None,
        TokenDestination::File => {
            if extras.is_empty() {
                let level = level.map(|level| format!("{level}:")).unwrap_or_default();
                return Err(ParseError::new(format!(
                    "File log destination requires a path. Example: `--log {level}{DEST_FILE}:/path/to/file`"
                )));
            }
            // If we find multiple extras, then the file path was split by a colon. Reconnect it.
            let path = extras.join(":");
            if path.is_empty() {
                let level = level.map(|level| format!("{level}:")).unwrap_or_default();
                return Err(ParseError::new(format!(
                    "File log destination requires a path. Example: `--log {level}{DEST_FILE}:/path/to/file`"
                )));
            }
            Some(LogDestination::File(path.into()))
        }
    };
    Ok(destination)
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
            (LevelFilter::Error, LEVEL_ERROR_UPPER),
            (LevelFilter::Warn, LEVEL_WARN),
            (LevelFilter::Warn, LEVEL_WARN_UPPER),
            (LevelFilter::Info, LEVEL_INFO),
            (LevelFilter::Info, LEVEL_INFO_UPPER),
            (LevelFilter::Debug, LEVEL_DEBUG),
            (LevelFilter::Debug, LEVEL_DEBUG_UPPER),
            (LevelFilter::Trace, LEVEL_TRACE),
            (LevelFilter::Trace, LEVEL_TRACE_UPPER)
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
    fn test_none_without_level() {
        let config = parse_config_definition("none").unwrap();
        assert_eq!(None, config);
    }

    #[apply(level)]
    #[rstest]
    fn test_none_with_level(level: (LevelFilter, &str)) {
        let config = parse_config_definition(&format!("{}:none", level.1)).unwrap();
        assert_eq!(None, config);
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

    #[test]
    fn file_destination_without_level() {
        let config = parse_config_definition("file:/path/to/file")
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: None,
                destination: LogDestination::File("/path/to/file".into()),
            },
            config,
        );
    }

    #[apply(level)]
    #[rstest]
    fn file_destination_with_level(level: (LevelFilter, &str)) {
        let config = parse_config_definition(&format!("{}:file:/path/to/file", level.1))
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: Some(level.0),
                destination: LogDestination::File("/path/to/file".into()),
            },
            config,
        );
    }

    #[test]
    fn file_destination_without_level_with_path_with_colons() {
        let config = parse_config_definition("file:/path/:to/:file")
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: None,
                destination: LogDestination::File("/path/:to/:file".into()),
            },
            config,
        );
    }

    #[apply(level)]
    #[rstest]
    fn file_destination_with_level_with_path_with_colons(level: (LevelFilter, &str)) {
        let config = parse_config_definition(&format!("{}:file:/path/:to/:file", level.1))
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: Some(level.0),
                destination: LogDestination::File("/path/:to/:file".into()),
            },
            config,
        );
    }

    #[test]
    fn file_destination_without_level_with_empty_path_2colons() {
        let config = parse_config_definition("file::").unwrap().unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: None,
                destination: LogDestination::File(":".into()),
            },
            config,
        );
    }

    #[apply(level)]
    #[rstest]
    fn file_destination_with_level_with_empty_path_2colons(level: (LevelFilter, &str)) {
        let config = parse_config_definition(&format!("{}:file::", level.1))
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: Some(level.0),
                destination: LogDestination::File(":".into()),
            },
            config,
        );
    }

    #[test]
    fn file_destination_without_level_with_empty_path_3colons() {
        let config = parse_config_definition("file:::").unwrap().unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: None,
                destination: LogDestination::File("::".into()),
            },
            config,
        );
    }

    #[apply(level)]
    #[rstest]
    fn file_destination_with_level_with_empty_path_3colons(level: (LevelFilter, &str)) {
        let config = parse_config_definition(&format!("{}:file:::", level.1))
            .unwrap()
            .unwrap();
        assert_eq!(
            LogDestinationConfig {
                level: Some(level.0),
                destination: LogDestination::File("::".into()),
            },
            config,
        );
    }

    mod errors {
        use super::*;

        #[test]
        fn empty() {
            let error = parse_config_definition("").unwrap_err();
            assert_eq!(
                "Invalid empty log destination. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[apply(level)]
        #[rstest]
        fn empty_destination_with_filter(level: (LevelFilter, &str)) {
            let error = parse_config_definition(&format!("{}:", level.1)).unwrap_err();
            assert_eq!(
                "Invalid empty log destination. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[apply(destination)]
        #[rstest]
        fn empty_filter(destination: (LogDestination, &str)) {
            let error = parse_config_definition(&format!(":{}", destination.1)).unwrap_err();
            assert_eq!(
                "Invalid empty log level filter. Choose ERROR, WARN, INFO, DEBUG, or TRACE",
                error.to_string()
            );
        }

        #[apply(level)]
        #[rstest]
        fn only_filter(level: (LevelFilter, &str)) {
            let error = parse_config_definition(level.1).unwrap_err();
            assert_eq!(
                format!(
                    "Expected log destination but found level filter `{}`. Please add a destination. Example: `--log {}:stderr`",
                    level.1, level.1,
                ),
                error.to_string()
            );
        }

        #[apply(destination)]
        #[rstest]
        fn invalid_level(destination: (LogDestination, &str)) {
            let error = parse_config_definition(&format!("invalid:{}", destination.1)).unwrap_err();
            assert_eq!(
                "Invalid log level filter `invalid`. Choose ERROR, WARN, INFO, DEBUG, or TRACE",
                error.to_string()
            );
        }

        #[apply(level)]
        #[rstest]
        fn invalid_destination_with_level(level: (LevelFilter, &str)) {
            let error = parse_config_definition(&format!("{}:invalid", level.1)).unwrap_err();
            assert_eq!(
                "Invalid log destination `invalid`. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[test]
        fn missing_colon() {
            let error =
                parse_config_definition(&format!("{LEVEL_ERROR_UPPER}{DEST_STDERR}")).unwrap_err();
            assert_eq!(
                "Invalid log destination `ERRORstderr`. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[test]
        fn partially_matching_filter() {
            // Regression test. A previous version misparsed this as a filter since it started like the error filter with 'E' and matched the first letter, but we should actually treat this as an invalid log destination
            let error = parse_config_definition("ega").unwrap_err();
            assert_eq!(
                "Invalid log destination `ega`. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[test]
        fn invalid_destination_without_filter() {
            let error = parse_config_definition("invalid").unwrap_err();
            assert_eq!(
                "Invalid log destination `invalid`. Choose stderr, syslog, file, or none",
                error.to_string()
            );
        }

        #[test]
        fn file_destination_without_level_without_path() {
            let error = parse_config_definition(&format!("{DEST_FILE}")).unwrap_err();
            assert_eq!(
                "File log destination requires a path. Example: `--log file:/path/to/file`",
                error.to_string()
            );
        }

        #[apply(level)]
        #[rstest]
        fn file_destination_with_level_without_path(level: (LevelFilter, &str)) {
            let error = parse_config_definition(&format!("{}:file", level.1)).unwrap_err();
            assert_eq!(
                format!(
                    "File log destination requires a path. Example: `--log {}:file:/path/to/file`",
                    level.1,
                ),
                error.to_string()
            );
        }

        #[test]
        fn file_destination_without_level_with_empty_path() {
            let error = parse_config_definition("file:").unwrap_err();
            assert_eq!(
                "File log destination requires a path. Example: `--log file:/path/to/file`",
                error.to_string()
            );
        }

        #[apply(level)]
        #[rstest]
        fn file_destination_with_level_with_empty_path(level: (LevelFilter, &str)) {
            let error = parse_config_definition(&format!("{}:file:", level.1)).unwrap_err();
            assert_eq!(
                format!(
                    "File log destination requires a path. Example: `--log {}:file:/path/to/file`",
                    level.1
                ),
                error.to_string()
            );
        }

        #[test]
        fn multiple_invalid_tokens() {
            let error = parse_config_definition("in:valid").unwrap_err();
            assert_eq!(
                "Invalid log configuration `in:valid`. Examples: `stderr`, `ERROR:syslog`, `WARN:file:/path/to/file`",
                error.to_string()
            );
        }
    }
}
