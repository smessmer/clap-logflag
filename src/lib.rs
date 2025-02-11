//! [work in progress]
//!
//! The [clap-logflag](https://crates.io/crates/clap-logflag) library adds a `--log` flag to clap based applications
//! to allow CLI users to configure logging from the command line.
//! It can log to stderr, files and syslog.

// TODO Enforce doc comments, no unsafe, ...

mod clap;
mod config;
mod fern;
mod parser;

pub use clap::LogArgs;
pub use config::{LogDestination, LogDestinationConfig, LoggingConfig};
pub use fern::_init_logging;
