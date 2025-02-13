use clap::Parser;
use clap_logflag::LoggingConfig;
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
            // If no `--log` arguments are present, disable logging
            .or_default(LoggingConfig::LoggingDisabled),
        LOG_DEFAULT_LEVEL
    );

    // Issue some log messages
    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
