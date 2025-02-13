use clap::Parser;
use clap_logflag::{LogDestinationConfig, LoggingConfig};
use log::LevelFilter;

const LOG_DEFAULT_LEVEL: LevelFilter = LevelFilter::Info;

#[derive(Debug, Parser)]
struct CliArgs {
    // Use this to add the log flags to your application
    #[clap(flatten)]
    log: clap_logflag::LogArgs,
}

fn main() {
    let args = CliArgs::parse();

    // Initialize logging with the flags from clap
    clap_logflag::init_logging!(
        args.log
            // If no `--log` arguments are present, log to stderr with the default level filter
            // Note that if the user passes in `--log none`, this will not trigger the default
            // and logging will be disabled instead. The default is only used if no `--log`
            // arguments are present.
            .or_default(LoggingConfig::LoggingEnabled {
                destinations: vec![LogDestinationConfig {
                    destination: clap_logflag::LogDestination::Stderr,
                    level: None,
                }],
            }),
        LOG_DEFAULT_LEVEL
    );

    // Issue some log messages
    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
