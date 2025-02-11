use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(flatten)]
    log: clap_logflag::LogArgs,
}

fn main() {
    let args = CliArgs::parse();
    println!("Logging Config: {:#?}", args);
}
