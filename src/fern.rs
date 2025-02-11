use std::io::IsTerminal as _;

use anyhow::Result;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch, FormatCallback,
};

use super::config::{LogDestination, LogDestinationConfig, LoggingConfig};

/// TODO Documentation
#[macro_export]
macro_rules! init_logging {
    ($config:expr, $default_level:expr) => {{
        $crate::_init_logging(
            $config.into(),
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
    match config {
        LoggingConfig::LoggingDisabled => Ok(()),
        LoggingConfig::LoggingEnabled { destinations } => {
            let process_name = process_name(cargo_bin_name, cargo_crate_name);

            let mut main_logger = Dispatch::new();
            for destination in destinations {
                if let Ok(logger) = build_logger(destination, default_level, process_name.clone()) {
                    main_logger = main_logger.chain(logger);
                }
            }
            main_logger.apply()?;
            Ok(())
        }
    }
}

fn build_logger(
    config: LogDestinationConfig,
    default_level: log::LevelFilter,
    process_name: String,
) -> Result<Dispatch> {
    let logger = Dispatch::new().level(config.level.unwrap_or(default_level));
    let logger = match &config.destination {
        LogDestination::Stderr => logger.format(log_formatter_stderr).chain(std::io::stderr()),
        LogDestination::File(path) => logger
            .format(log_formatter_file)
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

fn log_formatter_stderr(out: FormatCallback, message: &std::fmt::Arguments, record: &log::Record) {
    if std::io::stderr().is_terminal() {
        log_formatter_tty(out, message, record)
    } else {
        log_formatter_file(out, message, record)
    }
}

fn log_formatter_tty(out: FormatCallback, message: &std::fmt::Arguments, record: &log::Record) {
    let colors = ColoredLevelConfig::new()
        .trace(Color::Magenta)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    out.finish(format_args!(
        "[{} {} {}] {}",
        humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
        colors.color(record.level()),
        record.target(),
        message
    ))
}

fn log_formatter_file(out: FormatCallback, message: &std::fmt::Arguments, record: &log::Record) {
    out.finish(format_args!(
        "[{} {} {}] {}",
        humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
        record.level(),
        record.target(),
        message
    ))
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
