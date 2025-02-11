use clap::Parser;
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
    clap_logflag::init_logging!(args.log, args.default_level);

    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
