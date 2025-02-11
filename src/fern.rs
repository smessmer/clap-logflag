use anyhow::Result;
use fern::Dispatch;

use super::config::{LogDestination, LogDestinationConfig, LoggingConfig};

/// TODO Documentation
#[macro_export]
macro_rules! init_logging {
    ($config:expr, $default_level:expr) => {{
        $crate::_init_logging(
            $config,
            $default_level,
            option_env!("CARGO_BIN_NAME"),
            env!("CARGO_CRATE_NAME"),
        );
    }};
}

/// Don't use this function directly, use the [init_logging!] macro instead.
pub fn _init_logging(
    config: LoggingConfig,
    default_level: log::LevelFilter,
    cargo_bin_name: Option<&str>,
    cargo_crate_name: &str,
) {
    match config {
        LoggingConfig::LoggingDisabled => (),
        LoggingConfig::LoggingEnabled { destinations } => {
            let process_name = process_name(cargo_bin_name, cargo_crate_name);

            let mut main_logger = Dispatch::new();
            for destination in destinations {
                if let Ok(logger) = build_logger(destination, default_level, process_name.clone()) {
                    main_logger = main_logger.chain(logger);
                }
            }
            main_logger.apply().unwrap();
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
        LogDestination::Stderr => logger
            .format(move |out, message, record| {
                // TODO Better format, i.e. with time, and colored.
                out.finish(format_args!("[{}] {}", record.level(), message,))
            })
            .chain(std::io::stderr()),
        LogDestination::File(path) => logger.chain(fern::log_file(path)?),
        LogDestination::Syslog => {
            let syslog_formatter = syslog::Formatter3164 {
                facility: syslog::Facility::LOG_USER,
                hostname: None,
                process: process_name,
                pid: 0,
            };
            logger.chain(syslog::unix(syslog_formatter)?)
        }
    };
    Ok(logger)
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
