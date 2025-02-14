//! This example is not a real world example, it is used in our integration tests as a test binary.

use clap::Parser;
use clap_logflag::{LogDestinationConfig, LoggingConfig};
use log::LevelFilter;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Use this to add the log flags to your application
    #[clap(flatten)]
    log: clap_logflag::LogArgs,

    /// A real cli app would likely hardcode a default level instead of allowing users to pass it in.
    /// We're just doing that here to make this example useful for our integration tests.
    #[arg(long)]
    default_level: LevelFilter,
}

fn main() {
    let args = CliArgs::parse();

    // Initialize logging with the flags from clap
    clap_logflag::init_logging!(
        args.log
            // If no `--log` arguments are present, log to stderr but only log warnings and errors.
            .or_default(LoggingConfig::new(vec![LogDestinationConfig {
                destination: clap_logflag::LogDestination::Stderr,
                level: Some(LevelFilter::Warn),
            },],)),
        args.default_level
    );

    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
