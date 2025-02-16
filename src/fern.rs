use std::io::IsTerminal as _;

use anyhow::Result;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch, FormatCallback,
};

use super::config::{LogDestination, LogDestinationConfig, LoggingConfig};

/// Initialize logging with the given configuration and default level.
///
/// # Arguments
/// * `config` - The logging configuration to use.
/// * `default_level` - The default log level to use if a destination was specified without a log level filter.
///
/// # Example
/// ```rust
#[doc = include_str!("../examples/simple_cli.rs")]
/// ```
#[macro_export]
macro_rules! init_logging {
    ($config:expr, $default_level:expr $(,)?) => {{
        $crate::_init_logging(
            $config,
            $default_level,
            option_env!("CARGO_BIN_NAME"),
            env!("CARGO_CRATE_NAME"),
        )
        .expect("Failed to initialize logging");
    }};
}

/// Don't use this function directly, use the [init_logging!] macro instead.
pub fn _init_logging(
    config: LoggingConfig,
    default_level: log::LevelFilter,
    cargo_bin_name: Option<&str>,
    cargo_crate_name: &str,
) -> Result<()> {
    if config.destinations().is_empty() {
        // Logging is disabled
        return Ok(());
    }

    let process_name = process_name(cargo_bin_name, cargo_crate_name);

    let mut main_logger = Dispatch::new();
    for destination in config.destinations() {
        if let Ok(logger) = build_logger(destination, default_level, process_name.clone()) {
            main_logger = main_logger.chain(logger);
        }
    }
    main_logger.apply()?;
    Ok(())
}

fn build_logger(
    config: &LogDestinationConfig,
    default_level: log::LevelFilter,
    process_name: String,
) -> Result<Dispatch> {
    let logger = Dispatch::new().level(config.level.unwrap_or(default_level));
    let logger = match &config.destination {
        LogDestination::Stderr => {
            if std::io::stderr().is_terminal() {
                logger.format(log_formatter_tty()).chain(std::io::stderr())
            } else {
                logger.format(log_formatter_file()).chain(std::io::stderr())
            }
        }
        LogDestination::File(path) => logger
            .format(log_formatter_file())
            .chain(fern::log_file(path)?),
        LogDestination::Syslog => {
            let syslog_formatter = syslog::Formatter3164 {
                facility: syslog::Facility::LOG_USER,
                hostname: None,
                process: process_name,
                pid: std::process::id(),
            };
            logger.chain(syslog::unix(syslog_formatter)?)
        }
    };
    Ok(logger)
}

fn log_formatter_tty() -> impl Fn(FormatCallback, &std::fmt::Arguments, &log::Record) {
    let colors = ColoredLevelConfig::new()
        .trace(Color::Magenta)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    move |out: FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
            colors.color(record.level()),
            record.target(),
            message
        ))
    }
}

fn log_formatter_file() -> impl Fn(FormatCallback, &std::fmt::Arguments, &log::Record) {
    move |out: FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    }
}

/// Get a process name. Try in the following order:
/// 1. Try getting it from argv, i.e. the name of the currently running executable
/// 2. Try getting it from the `CARGO_BIN_NAME` environment variable
/// 3. Get it from the `CARGO_CRATE_NAME` environment variable
fn process_name(cargo_bin_name: Option<&str>, cargo_crate_name: &str) -> String {
    exe_name()
        .unwrap_or_else(|| {
            cargo_bin_name
                .map(str::to_string)
                .unwrap_or_else(|| cargo_crate_name.to_string())
        })
        .to_string()
}

/// Get the currently running executable name from argv.
fn exe_name() -> Option<String> {
    std::env::current_exe()
        .map(|exe_path| {
            exe_path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_string)
        })
        .unwrap_or(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{Level, LevelFilter};
    use predicates::Predicate;
    use rstest::rstest;

    // TODO Check coverage and add tests for code uncovered by unit tests

    #[test]
    fn test_exe_name() {
        let actual_exe_name = exe_name().unwrap();
        assert!(
            actual_exe_name.starts_with("clap_logflag"),
            "exe_name should start with clap_logflag but was {actual_exe_name}"
        );
    }

    #[test]
    fn test_process_name() {
        let actual_process_name = process_name(None, "cargo_crate_name");
        assert!(
            actual_process_name.starts_with("clap_logflag"),
            "process_name should start with clap_logflag but was {actual_process_name}"
        );
    }

    #[rstest]
    fn test_build_logger(
        #[values(LogDestination::Stderr, LogDestination::Syslog)] destination: LogDestination,
        #[values(
            LevelFilter::Error,
            LevelFilter::Warn,
            LevelFilter::Info,
            LevelFilter::Debug,
            LevelFilter::Trace
        )]
        level: LevelFilter,
    ) {
        let config = LogDestinationConfig {
            destination,
            level: None,
        };
        let logger = build_logger(&config, level, "process_name".to_string())
            .unwrap()
            .into_log();
        assert_eq!(logger.0, level);
    }

    #[rstest]
    fn test_build_file_logger(
        #[values(
            LevelFilter::Error,
            LevelFilter::Warn,
            LevelFilter::Info,
            LevelFilter::Debug,
            LevelFilter::Trace
        )]
        level: LevelFilter,
    ) {
        let tempdir = assert_fs::TempDir::new().unwrap();
        let file = tempdir.path().join("logfile");
        let config = LogDestinationConfig {
            destination: LogDestination::File(file),
            level: None,
        };
        let logger = build_logger(&config, level, "process_name".to_string())
            .unwrap()
            .into_log();
        assert_eq!(logger.0, level);
    }

    #[rstest]
    fn test_log_formatter_file() {
        let tempdir = assert_fs::TempDir::new().unwrap();
        let file = tempdir.path().join("logfile");
        let config = LogDestinationConfig {
            destination: LogDestination::File(file.clone()),
            level: None,
        };
        let (level, logger) = build_logger(&config, LevelFilter::Debug, "process_name".to_string())
            .unwrap()
            .into_log();
        assert_eq!(level, LevelFilter::Debug);
        logger.log(
            &log::Record::builder()
                .args(format_args!("test log message"))
                .level(Level::Debug)
                .target("my-test")
                .line(Some(1))
                .build(),
        );
        logger.flush();

        let timestamp_regex = r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)";
        let expected_log_regex =
            format!(r"\[{timestamp_regex} {level} my-test\] test log message\n");

        let actually_logged = std::fs::read_to_string(&file).unwrap();
        // Assert it matches
        assert!(
            predicates::str::is_match(expected_log_regex)
                .unwrap()
                .eval(&actually_logged),
            "actually_logged: \"{actually_logged}\""
        );
    }
}
