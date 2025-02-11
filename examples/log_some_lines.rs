use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(flatten)]
    log: clap_logflag::LogArgs,
}

fn main() {
    let args = CliArgs::parse();
    clap_logflag::init_logging(args.log.into(), LevelFilter::Info, "app name");

    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some info log");
    log::warn!("Some warn log");
    log::error!("Some error log");
}
