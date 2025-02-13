use clap::Parser;
use clap_logflag::LoggingConfig;
use log::LevelFilter;

#[derive(Debug, Parser)]
struct CliArgs {
    // Use this to add the log flags to your application
    #[clap(flatten)]
    log: clap_logflag::LogArgs,
    //
    // ... your other cli args ...
}

fn main() {
    let args = CliArgs::parse();

    // Initialize logging with the flags from clap
    clap_logflag::init_logging!(
        args.log
            // If no `--log` arguments are present, disable logging.
            // You can change this to define the default behavior,
            // see the "default_logging" example.
            .or_default(LoggingConfig::LoggingDisabled),
        // Any `--log` argument that doesn't define a level filter will use the
        // default level filter defined here, `Info` in this example.
        LevelFilter::Info,
    );

    // Issue some log messages
    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
