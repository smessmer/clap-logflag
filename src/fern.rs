use anyhow::Result;
use fern::Dispatch;

use super::config::{LogDestination, LogDestinationConfig, LoggingConfig};

// TODO Get app_name from argv instead of as argument
pub fn init_logging(config: LoggingConfig, app_name: &str) {
    match config {
        LoggingConfig::LoggingDisabled => (),
        LoggingConfig::LoggingEnabled { destinations } => {
            let mut main_logger = Dispatch::new();
            for destination in destinations {
                if let Ok(logger) = build_logger(destination, app_name) {
                    main_logger = main_logger.chain(logger);
                }
            }
            main_logger.apply().unwrap();
        }
    }
}

fn build_logger(config: LogDestinationConfig, app_name: &str) -> Result<Dispatch> {
    let logger = Dispatch::new().level(config.level);
    let logger = match &config.destination {
        LogDestination::Stderr => logger
            .format(move |out, message, record| {
                out.finish(format_args!("[{}] {}", record.level(), message,))
            })
            .chain(std::io::stderr()),
        LogDestination::File(path) => logger.chain(fern::log_file(path)?),
        LogDestination::Syslog => {
            let syslog_formatter = syslog::Formatter3164 {
                facility: syslog::Facility::LOG_USER,
                hostname: None,
                process: app_name.to_owned(),
                pid: 0,
            };
            logger.chain(syslog::unix(syslog_formatter)?)
        }
    };
    Ok(logger)
}
