use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(flatten)]
    log: clap_logflag::LogArgs,

    /// A real cli app would not allow users to pass in the default level but hardcode it.
    /// We're just doing that here to make this example useful for our integration tests.
    #[clap(long)]
    default_level: LevelFilter,
}

fn main() {
    let args = CliArgs::parse();
    clap_logflag::init_logging(args.log.into(), args.default_level, "app name");

    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
